//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 953/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk953(t45889: f64, t7720: f64, t10088: f64, t495: f64, t511: f64, t7230: f64, t7231: f64, t1737: f64, t3351: f64, t498: f64, t880: f64, t3352: f64, t6394: f64) -> (f64, f64, f64, f64) {
    let t45890 = t7720 * t45889;
    let t45896 = t7230 * t7231 * t511 * t10088 * t495;
    let t45901 = t3351 * t7231 * t880 * t1737 * t498;
    let t45905 = t3351 * t3352 * t880 * t6394;
    (t45890, t45896, t45901, t45905)
}
