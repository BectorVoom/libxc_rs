//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1254/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1254<F: Float>(t14607: F, t524: F, t547: F, t4346: F, t4495: F, t10334: F, t195: F, t217: F, t1006: F, t10335: F, t1139: F, t15711: F, t15721: F, t285: F, t288: F, t2934: F) -> (F, F, F, F, F, F, F) {
    let t42957 = t524 / t14607 / t547;
    let t43108 = t4495 * t4346;
    let t43141 = t195 / t10334 / t217;
    let t43151 = t1006 * t10335;
    let t43179 = t15711 * t1139;
    let t43184 = t285 / t15721 / t288;
    let t43191 = t2934 * t2934;
    (t42957, t43108, t43141, t43151, t43179, t43184, t43191)
}
