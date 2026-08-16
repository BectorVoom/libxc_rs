//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 320/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk320<F: Float>(t262: F, t3080: F, t2079: F, t3046: F, t793: F, t637: F, t797: F, t1322: F, t838: F, t1326: F) -> (F, F, F, F, F) {
    let t3081 = t262 * t3080;
    let t3082 = t2079 * t3081;
    let t3088 = t793 * t3046;
    let t3091 = t797 * t3046 * t637;
    let t3093 = t838 * t1322;
    let t3094 = t1326 * t3046;
    (t3082, t3088, t3091, t3093, t3094)
}
