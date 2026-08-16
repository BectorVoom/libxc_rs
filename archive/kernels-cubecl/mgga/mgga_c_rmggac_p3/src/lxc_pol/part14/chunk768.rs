//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 768/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk768<F: Float>(t2078: F, t3851: F, t7834: F, t797: F, t321: F, t7840: F, t5259: F, t333: F, t4669: F, t128: F, t305: F, t3899: F) -> (F, F, F, F, F, F, F) {
    let t35815 = t3851 * t2078;
    let t35824 = t797 * t7834;
    let t35844 = t7840 * t321;
    let t35845 = t5259 * t35844;
    let t35847 = t7840 * t333;
    let t35848 = t4669 * t35847;
    let t35861 = t305 * t128 * t3899;
    (t35815, t35824, t35844, t35845, t35847, t35848, t35861)
}
