//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1317/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1317<F: Float>(t713: F, t9462: F, t1976: F, t3586: F, t1954: F, t694: F, t9515: F, t1108: F, t17351: F, t17354: F, t17621: F, t17664: F, t1957: F, t1972: F, t1980: F, t20705: F, t20716: F, t20719: F, t20834: F, t20837: F, t20908: F, t21093: F, t21226: F, t248: F, t25609: F, t25633: F, t25636: F, t25639: F, t25691: F, t25703: F, t25816: F, t2829: F, t2848: F, t2849: F, t3592: F, t704: F, t723: F, t7241: F, t7255: F, t7300: F, t7475: F, t7478: F, t9499: F, t9532: F) -> (F,) {
    let t26048 = t9462 * t713;
    let t26053 = t3586 * t1976;
    let t26062 = t3586 * t1954;
    let t26065 = t9515 * t694;
    let t26070 = -t25609 + 0.41016075432865626631e4 * t20837 * t9532 * t2848 - 0.4155806185363551302e3 * t20834 * t7300 + 0.14035736694323150897e2 * t21093 * t7255 - 0.77193501593724168323e3 * t20908 * t7241 - 0.310907e-1 * (t17664 - 0.10654518518518518518e0 * t17351 + 0.22831111111111111111e-1 * t17354 - 0.10654518518518518518e0 * t20705 + 0.91324444444444444442e-1 * t20716 - 0.34246666666666666666e-1 * t20719 + 0.22831111111111111111e-1 * t25633 - 0.34246666666666666666e-1 * t25636 + 0.5137e-1 * t25639) * t248 + t25691 - t25703 + 0.11696447245269292414e1 * t26048 * t723 + 0.5848223622634646207e0 * t9499 * t1972 + 0.17315859105681463759e2 * t26053 * t1980 + 0.11696447245269292414e1 * t21226 * t1108 + 0.23392894490538584828e1 * t7478 * t2849 + 0.11696447245269292414e1 * t2829 * t7475 - 0.11696447245269292414e1 * t26062 * t1957 + 2.0 * t26065 * t704 - 0.11696447245269292414e1 * t17621 * t3592 - t25816;
    (t26070,)
}
