//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 953/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk953<F: Float>(t45889: F, t7720: F, t10088: F, t495: F, t511: F, t7230: F, t7231: F, t1737: F, t3351: F, t498: F, t880: F, t3352: F, t6394: F) -> (F, F, F, F) {
    let t45890 = t7720 * t45889;
    let t45896 = t7230 * t7231 * t511 * t10088 * t495;
    let t45901 = t3351 * t7231 * t880 * t1737 * t498;
    let t45905 = t3351 * t3352 * t880 * t6394;
    (t45890, t45896, t45901, t45905)
}
