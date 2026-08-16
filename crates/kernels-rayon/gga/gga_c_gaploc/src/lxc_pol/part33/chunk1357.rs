//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1357/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1357(t2366: f64, t38276: f64, t12000: f64, t158: f64, t1063: f64, t12008: f64, t123: f64, t1349: f64, t1358: f64, t29850: f64, t29852: f64, t31488: f64, t31490: f64, t31493: f64, t31496: f64, t31498: f64, t31522: f64, t3808: f64, t38267: f64, t38272: f64, t38277: f64, t4323: f64, t488: f64, t535: f64, t6507: f64) -> (f64, f64, f64) {
    let t38281 = t2366 * t38276;
    let t38285 = t158 * t12000;
    let t38292 = -0.56910013271352299198e-1_f64 * t1063 * t535 * t38267 - 0.63233348079280332442e-2_f64 * t1349 * t4323 * t38272 - 0.12646669615856066488e-1_f64 * t1358 * t6507 * t38277 + 0.18970004423784099733e-1_f64 * t1358 * t4323 * t38281 - 0.63233348079280332442e-2_f64 * t1358 * t38285 * t123 * t488 + 0.63233348079280332442e-2_f64 * t3808 * t12008 - t31488 + t31490 + t31493 + t31496 + t29850 - t29852 + t31498 + t31522;
    (t38281, t38285, t38292)
}
