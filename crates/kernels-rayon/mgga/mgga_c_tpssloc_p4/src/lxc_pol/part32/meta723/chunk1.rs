//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2307/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2307(t24574: f64, t29702: f64, t103515: f64, t11907: f64, t1216: f64, t1716: f64, t18525: f64, t18946: f64, t19203: f64, t2148: f64, t24812: f64, t27489: f64, t27490: f64, t27492: f64, t27496: f64, t27507: f64, t27510: f64, t27536: f64, t27540: f64, t27732: f64, t29709: f64, t3610: f64, t6140: f64, t7283: f64, t7373: f64, t7381: f64, t8082: f64, t94858: f64, t95033: f64) -> f64 {
    let t103744 = t24574 * t29702;
    let t103766 = -0.16449340668482264365e-1_f64 * t7373 * t27536 * t27510 + 4.0_f64 * t3610 * t8082 * t19203 - 0.16449340668482264365e-1_f64 * t7283 * t1716 * t27732 - 0.27415567780803773942e-2_f64 * t103744 - t11907 * t29709 + 0.36554090374405031923e-2_f64 * t95033 - 0.82246703342411321825e-2_f64 * t7283 * t18525 * t2148 - 0.82246703342411321825e-2_f64 * t7283 * t6140 * t7381 - 0.82246703342411321825e-2_f64 * t24812 * t27496 * t103515 * t1216 + 0.3289868133696452873e-1_f64 * t24812 * t27489 * t27490 * t18946 + 0.43864908449286038306e-1_f64 * t27507 * t27540 - 0.87729816898572076612e-1_f64 * t94858 * t27492;
    t103766
}
