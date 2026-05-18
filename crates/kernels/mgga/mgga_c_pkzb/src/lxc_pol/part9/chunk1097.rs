//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1097/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1097<F: Float>(t154: F, t18086: F, t1885: F, t276: F, t5645: F, t735: F, t2899: F, t5704: F, t774: F, t2922: F, t5961: F, t5975: F, t5984: F) -> (F, F, F, F, F) {
    let t18089 = t276 * t154 * t18086 * t1885;
    let t18091 = t735 * t5645;
    let t18094 = t2899 * t774 * t5704;
    let t18097 = t2922 * t774 * t5961;
    let t18103 = t5984 * t5975;
    (t18089, t18091, t18094, t18097, t18103)
}
