//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 610/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk610<F: Float>(t2846: F, t273: F, t2881: F, t2889: F, t923: F, t240: F, t68: F, t281: F, t283: F, t698: F, t931: F) -> (F, F, F, F, F, F, F, F) {
    let t2892 = F::cast_from(0.39862222222222222223e0_f64) * t2846;
    let t2897 = F::cast_from(1.0_f64)/F::sqrt(t273);
    let t2898 = t2897 * t2881;
    let t2900 = t923 * t2889;
    let t2902 = t68 * t240;
    let t2904 = t281 * t2902 * t283;
    let t2905 = F::cast_from(0.13692777777777777778e0_f64) * t2904;
    let t2906 = t698 * t931;
    (t2892, t2897, t2898, t2900, t2902, t2904, t2905, t2906)
}
