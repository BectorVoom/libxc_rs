//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2961/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2961<F: Float>(t1087: F, t43065: F, t3105: F, t4857: F, t1012: F, t43222: F, t16190: F, t3173: F, t1011: F, t11714: F, t15144: F, t15830: F, t16012: F, t16095: F, t16096: F, t16196: F, t16223: F, t3092: F, t3101: F, t3106: F, t3130: F, t4803: F, t4919: F, t51851: F, t51856: F, t51925: F, t51930: F) -> (F, F) {
    let t53923 = t1087 * t43065;
    let t53926 = t4857 * t3105;
    let t53944 = t1012 * t43222;
    let t53948 = t16190 * t3173;
    let t53954 = -F::cast_from(0.76220476654346199061e-2_f64) * t53923 * t16223 + F::cast_from(0.45732285992607719436e-2_f64) * t53926 * t3130 + F::cast_from(0.45732285992607719436e-2_f64) * t15830 * t3101 + F::cast_from(0.91464571985215438873e-2_f64) * t11714 * t4803 + F::cast_from(0.45732285992607719436e-2_f64) * t3106 * t16196 + t1011 * t4919 * t51925 / F::new(72.0) + F::new(7.0) / F::new(216.0) * t1011 * t16012 * t51930 + t1011 * t4919 * t51851 / F::new(216.0) + F::new(35.0) / F::new(972.0) * t1011 * t53944 * t51856 - F::cast_from(0.45732285992607719436e-2_f64) * t53948 + F::cast_from(0.17149607247227894789e-2_f64) * t16095 * t3092 * t15144 * t16096;
    (t53923, t53954)
}
