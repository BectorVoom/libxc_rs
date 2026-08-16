//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1064/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1064(t2331: f64, t8288: f64, t14612: f64, t14665: f64, t21969: f64, t26755: f64, t26764: f64, t26785: f64, t26787: f64, t30236: f64, t30241: f64, t30244: f64, t30247: f64, t30254: f64, t30258: f64, t30262: f64, t30264: f64, t30266: f64, t4347: f64, t6426: f64, t8289: f64, t8404: f64) -> (f64, f64) {
    let t31438 = t8288 * t2331;
    let t31439 = t31438 * t14612;
    let t31450 = -0.34822083333333333333e-2_f64 * t26755 + 0.51588271604938271605e-2_f64 * t30236 + 0.11607361111111111111e-2_f64 * t30241 + 0.34822083333333333333e-2_f64 * t30244 + 0.34822083333333333333e-2_f64 * t30247 - 0.46429444444444444443e-2_f64 * t26764 + 0.46429444444444444443e-2_f64 * t26785 + 0.23214722222222222222e-2_f64 * t26787 + 0.69644166666666666665e-2_f64 * t30254 - 0.579e0_f64 * t6426 * t8404 - 0.223494e0_f64 * t4347 * t31439 + 0.223494e0_f64 * t21969 * t8289 + t14665 + 0.579e0_f64 * t6426 * t8289 + 0.23214722222222222222e-2_f64 * t30258 - 0.46429444444444444443e-2_f64 * t30262 - 0.69644166666666666665e-2_f64 * t30264 + 0.46429444444444444443e-2_f64 * t30266;
    (t31439, t31450)
}
