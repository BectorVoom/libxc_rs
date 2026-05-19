//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1326/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1326<F: Float>(t14630: F, t4768: F, t11493: F, t123: F, t1325: F, t1383: F, t17045: F, t2773: F, t2812: F, t323: F, t32722: F, t3813: F, t3836: F, t3884: F, t3885: F, t3917: F, t42092: F, t42382: F, t42427: F, t42487: F, t42490: F, t51589: F, t51701: F, t51729: F, t51733: F, t51736: F, t51743: F, t51747: F, t57628: F, t57770: F, t7865: F, t8201: F, t894: F, t953: F) -> (F, F) {
    let t57846 = t14630 * t4768;
    let t57852 = -F::cast_from(0.17581974682482873924e4_f64) * t3884 * t3885 * t3813 * t17045 + F::cast_from(0.35163949364965747848e4_f64) * t3917 * t3885 * t8201 * t17045 - F::cast_from(0.3863627328795003491e-1_f64) * t42382 - F::cast_from(0.1343485452223045261e0_f64) * t51589 - F::cast_from(0.1039653020352937208e2_f64) * t42427 + F::cast_from(0.7727254657590006982e-1_f64) * t42487 - F::cast_from(0.51515031050600046546e-1_f64) * t42490 + F::cast_from(0.18137053605011111024e0_f64) * t953 * t894 * t7865 * t57628 + F::cast_from(0.42929192542166705456e-1_f64) * t32722 + F::cast_from(0.26372962023724310886e4_f64) * t2773 * t323 * t57770 * t123 - F::cast_from(0.23181763972770020945e0_f64) * t51701 - F::cast_from(0.80609127133382715662e-1_f64) * t51729 + F::cast_from(0.15146801702008125515e1_f64) * t51733 + F::cast_from(0.519826510176468604e2_f64) * t51736 + F::cast_from(0.18583473745796456084e3_f64) * t11493 * t42092 * t1325 * t1383 + F::cast_from(0.389869882632351453e2_f64) * t2812 * t3836 * t57846 + F::cast_from(0.38636273287950034909e-1_f64) * t51743 + F::cast_from(0.31957282085435444036e5_f64) * t51747;
    (t57846, t57852)
}
