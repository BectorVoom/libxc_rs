//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 822/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk822<F: Float>(t301: F, t5913: F, t761: F, t758: F, t2011: F, t2099: F, t757: F, t2012: F, t2041: F, t2096: F, t2104: F, t2899: F, t2945: F, t5681: F, t5685: F, t5691: F, t5696: F, t5700: F, t5705: F, t5709: F, t5713: F, t5725: F, t5731: F) -> (F, F, F, F) {
    let t5915 = t301 * t5913 * t761;
    let t5916 = t758 * t5915;
    let t5921 = t2099 * t2011;
    let t5922 = t757 * t5921;
    let t5924 = -F::cast_from(0.85748036236139473944e-3_f64) * t5681 + F::cast_from(0.38586616306262763276e-2_f64) * t2945 * t5685 + t5691 / F::cast_from(144.0_f64) + F::cast_from(0.38586616306262763275e-2_f64) * t2104 * t5696 - F::cast_from(0.12862205435420921092e-2_f64) * t2104 * t5700 + F::cast_from(0.12862205435420921092e-2_f64) * t2899 * t5705 - F::cast_from(0.42874018118069736972e-3_f64) * t5709 + F::cast_from(0.34299214494455789577e-2_f64) * t5713 * t2041 - F::cast_from(0.12862205435420921092e-2_f64) * t5725 * t5731 + F::cast_from(0.21437009059034868486e-3_f64) * t757 * t5916 - F::cast_from(0.34299214494455789577e-2_f64) * t2096 * t2012 + F::cast_from(0.42874018118069736972e-3_f64) * t5922;
    (t5915, t5921, t5922, t5924)
}
