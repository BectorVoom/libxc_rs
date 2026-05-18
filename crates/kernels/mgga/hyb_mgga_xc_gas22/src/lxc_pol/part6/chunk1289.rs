//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1289/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1289<F: Float>(t1318: F, t3271: F, t10169: F, t2014: F, t684: F, t10204: F, t2028: F, t10205: F, t8498: F, t8526: F, t2002: F, t23889: F, t8511: F) -> (F, F, F, F, F, F) {
    let t27941 = t3271 * t1318;
    let t27955 = t684 * t2014 * t10169;
    let t27957 = t10204 * t2028;
    let t27962 = t8526 * t8498 * t10205;
    let t27968 = t10204 * t2002;
    let t27976 = t8511 * t23889 * t10205;
    (t27941, t27955, t27957, t27962, t27968, t27976)
}
