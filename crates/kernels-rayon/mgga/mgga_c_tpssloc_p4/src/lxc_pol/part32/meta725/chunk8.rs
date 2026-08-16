//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2337/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2337(t24574: f64, t29694: f64, t1170: f64, t2121: f64, t29670: f64, t29678: f64, t7280: f64, t14972: f64, t15820: f64, t1761: f64, t18571: f64, t2144: f64, t24893: f64, t27383: f64, t27396: f64, t27406: f64, t27427: f64, t29795: f64, t3487: f64, t4945: f64, t498: f64, t6150: f64, t6268: f64, t7348: f64, t8088: f64, t86451: f64, t94759: f64, t95899: f64) -> f64 {
    let t104509 = t24574 * t29694;
    let t104521 = t2121 * t1170 * t29670;
    let t104527 = t29678 * t7280;
    let t104534 = -0.18277045187202515961e-2_f64 * t104509 - 0.43864908449286038306e-1_f64 * t27406 * t27383 + 0.14621636149762012769e-1_f64 * t27406 * t27427 + 4.0_f64 * t4945 * t27396 - t3487 * t29795 - 2.0_f64 * t15820 * t8088 + 0.27415567780803773942e-2_f64 * t104521 + t18571 * t2144 * t498 + t6150 * t7348 * t498 + 0.26806332941230356743e-1_f64 * t104527 - 2.0_f64 * t95899 * t1761 - t24893 * t6268 - 2.0_f64 * t14972 * t8088 - t94759 + t86451;
    t104534
}
