//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 883/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk883<F: Float>(t4459: F, t512: F, t507: F, t12998: F, t12974: F, t1527: F, t4462: F, t515: F, t1524: F, t4435: F, t1197: F, t3696: F, t3716: F, t3725: F, t240: F, t3688: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14756 = 1.0 / t4459 / t512;
    let t14757 = t507 * t14756;
    let t14784 = 0.46308888888888888888e0 * t12998;
    let t14785 = 0.16068111111111111111e1 * t12974;
    let t14797 = 1.0 / t4459 / t1527;
    let t14798 = t507 * t14797;
    let t14800 = 1.0 / t4462 / t515;
    let t14804 = t1524 * t4435;
    let t14810 = t1197 * t3696;
    let t14827 = t3716 * t3725;
    let t14831 = 0.53272592592592592592e-1 * t12974;
    let t14850 = t240 * t3688;
    (t14757, t14784, t14785, t14798, t14800, t14804, t14810, t14827, t14831, t14850)
}
