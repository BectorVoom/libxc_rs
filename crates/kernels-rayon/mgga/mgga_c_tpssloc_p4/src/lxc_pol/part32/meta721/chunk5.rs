//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2295/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2295(t8015: f64, t94490: f64, t24574: f64, t29682: f64, t29691: f64, t24589: f64, t24880: f64, t27392: f64, t27406: f64, t27437: f64, t27761: f64, t29536: f64, t3487: f64, t4945: f64, t6146: f64, t6268: f64, t7283: f64, t7295: f64, t94475: f64, t94476: f64, t94492: f64, t94494: f64, t94514: f64, t94525: f64) -> f64 {
    let t103286 = t94490 * t8015;
    let t103291 = t24574 * t29682;
    let t103293 = t24574 * t29691;
    let t103303 = -t24880 * t6268 - t94475 + 0.36554090374405031923e-2_f64 * t94476 + t94492 + t94494 + 4.0_f64 * t4945 * t27761 + 0.14621636149762012769e-1_f64 * t103286 - 0.82246703342411321825e-2_f64 * t7283 * t6146 * t7295 - 0.27415567780803773942e-2_f64 * t103291 + 0.12184696791468343974e-2_f64 * t103293 - 0.43864908449286038306e-1_f64 * t27406 * t27392 - 0.54831135561607547883e-2_f64 * t24589 * t94514 * t27437 + 0.12184696791468343974e-2_f64 * t94525 + 2.0_f64 * t3487 * t29536;
    t103303
}
