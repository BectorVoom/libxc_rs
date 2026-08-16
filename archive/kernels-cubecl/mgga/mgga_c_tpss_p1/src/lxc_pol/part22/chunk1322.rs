//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1322/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1322<F: Float>(t1006: F, t3724: F, t8096: F, t19818: F, t20047: F, t44474: F, t18246: F, t64296: F, t14076: F, t61703: F, t1497: F, t2436: F) -> (F, F, F, F, F, F) {
    let t64954 = t1006 * t3724;
    let t64957 = t8096 * t1006;
    let t64958 = t64957 * t19818;
    let t64966 = t20047 * t44474;
    let t64969 = t18246 * t64296;
    let t64972 = t61703 * t14076;
    let t64975 = t2436 * t1497;
    (t64954, t64958, t64966, t64969, t64972, t64975)
}
