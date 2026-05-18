//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 899/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk899<F: Float>(t1375: F, t14267: F, t16816: F, t828: F, t837: F, t845: F, t3788: F, t4954: F, t4958: F, t10645: F, t14029: F, t1415: F, t16931: F, t16935: F, t16941: F, t16945: F, t16947: F, t3980: F) -> (F, F, F, F, F, F) {
    let t16949 = F::new(0.17544670192365612213e1) * t14267 * t1375;
    let t16951 = t828 * t16816 * t837;
    let t16953 = F::new(0.58482233974552040708e0) * t845 * t16951;
    let t16955 = F::new(0.17544670192365612213e1) * t3788 * t4954;
    let t16957 = F::new(0.51947267698127589899e2) * t3788 * t4958;
    let t16958 = -t10645 / F::new(3.0) + t16931 - t16935 - F::new(0.77534644304710291488e-2) * t3980 * t14029 * t1415 - t16941 - t16945 + t16947 - t16949 - t16953 - t16955 - t16957;
    (t16949, t16951, t16953, t16955, t16957, t16958)
}
