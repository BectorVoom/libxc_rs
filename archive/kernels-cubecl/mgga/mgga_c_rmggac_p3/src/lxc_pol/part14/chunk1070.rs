//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1070/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1070<F: Float>(t1475: F, t1970: F, t1971: F, t839: F, t880: F, t236: F, t794: F, t9188: F, t1614: F, t2084: F, t2139: F, t27: F) -> (F, F, F) {
    let t42109 = t1970 * t1971 * t880 * t1475 * t839;
    let t42114 = t1970 * t9188 * t236 * t1475 * t794;
    let t42132 = t2139 * t27 * t2084 * t1614;
    (t42109, t42114, t42132)
}
