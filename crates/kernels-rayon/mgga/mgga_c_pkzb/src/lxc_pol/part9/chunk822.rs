//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 822/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk822(t301: f64, t5913: f64, t761: f64, t758: f64, t2011: f64, t2099: f64, t757: f64, t2012: f64, t2041: f64, t2096: f64, t2104: f64, t2899: f64, t2945: f64, t5681: f64, t5685: f64, t5691: f64, t5696: f64, t5700: f64, t5705: f64, t5709: f64, t5713: f64, t5725: f64, t5731: f64) -> (f64, f64, f64, f64) {
    let t5915 = t301 * t5913 * t761;
    let t5916 = t758 * t5915;
    let t5921 = t2099 * t2011;
    let t5922 = t757 * t5921;
    let t5924 = -0.85748036236139473944e-3_f64 * t5681 + 0.38586616306262763276e-2_f64 * t2945 * t5685 + t5691 / 144.0_f64 + 0.38586616306262763275e-2_f64 * t2104 * t5696 - 0.12862205435420921092e-2_f64 * t2104 * t5700 + 0.12862205435420921092e-2_f64 * t2899 * t5705 - 0.42874018118069736972e-3_f64 * t5709 + 0.34299214494455789577e-2_f64 * t5713 * t2041 - 0.12862205435420921092e-2_f64 * t5725 * t5731 + 0.21437009059034868486e-3_f64 * t757 * t5916 - 0.34299214494455789577e-2_f64 * t2096 * t2012 + 0.42874018118069736972e-3_f64 * t5922;
    (t5915, t5921, t5922, t5924)
}
