//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1064/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1064<F: Float>(t2331: F, t8288: F, t14612: F, t14665: F, t21969: F, t26755: F, t26764: F, t26785: F, t26787: F, t30236: F, t30241: F, t30244: F, t30247: F, t30254: F, t30258: F, t30262: F, t30264: F, t30266: F, t4347: F, t6426: F, t8289: F, t8404: F) -> (F, F) {
    let t31438 = t8288 * t2331;
    let t31439 = t31438 * t14612;
    let t31450 = -F::cast_from(0.34822083333333333333e-2_f64) * t26755 + F::cast_from(0.51588271604938271605e-2_f64) * t30236 + F::cast_from(0.11607361111111111111e-2_f64) * t30241 + F::cast_from(0.34822083333333333333e-2_f64) * t30244 + F::cast_from(0.34822083333333333333e-2_f64) * t30247 - F::cast_from(0.46429444444444444443e-2_f64) * t26764 + F::cast_from(0.46429444444444444443e-2_f64) * t26785 + F::cast_from(0.23214722222222222222e-2_f64) * t26787 + F::cast_from(0.69644166666666666665e-2_f64) * t30254 - F::cast_from(0.579e0_f64) * t6426 * t8404 - F::cast_from(0.223494e0_f64) * t4347 * t31439 + F::cast_from(0.223494e0_f64) * t21969 * t8289 + t14665 + F::cast_from(0.579e0_f64) * t6426 * t8289 + F::cast_from(0.23214722222222222222e-2_f64) * t30258 - F::cast_from(0.46429444444444444443e-2_f64) * t30262 - F::cast_from(0.69644166666666666665e-2_f64) * t30264 + F::cast_from(0.46429444444444444443e-2_f64) * t30266;
    (t31439, t31450)
}
