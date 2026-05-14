//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1348/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1348<F: Float>(t3135: F, t889: F, t1209: F, t22185: F, t22684: F, t22841: F, t851: F, t22503: F, t3139: F, t22820: F, t3038: F, t22722: F, t3074: F, t8009: F, t8189: F, t8199: F, t8219: F) -> (F, F, F, F, F, F, F, F) {
    let t26848 = t889 * t3135;
    let t26851 = 0.14035736694323150897e2 * t22185 * t1209 * t26848;
    let t26854 = 0.2069040516770936012e4 * t22684 * t22841 * t851;
    let t26857 = 0.4155806185363551302e3 * t22503 * t3139 * t26848;
    let t26859 = 8.0 * t22820 * t3038;
    let t26861 = 0.64327917994770140268e2 * t22722 * t3074;
    let t26863 = 8.0 * t8009 * t8189;
    let t26865 = 0.64327917994770140268e2 * t8219 * t8199;
    (t26848, t26851, t26854, t26857, t26859, t26861, t26863, t26865)
}
