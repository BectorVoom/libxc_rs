//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 926/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk926<F: Float>(t12069: F, t12089: F, t12107: F, t12225: F, t354: F, t11866: F, t11876: F, t11886: F, t11035: F, t11046: F, t11052: F, t11206: F, t11215: F, t11868: F, t11870: F, t11872: F, t11874: F, t11878: F, t11883: F, t11889: F) -> (F, F, F) {
    let t12227 = t12069 + t12089 + t12107 + t12225;
    let t12228 = t354 * t12227;
    let t12230 = 2.0 / 3.0 * t11866;
    let t12235 = 2.0 / 3.0 * t11876;
    let t12238 = 4.0 / 3.0 * t11886;
    let t12240 = -t11206 - t11035 - t12230 - t11868 / 2.0 + t11870 / 4.0 - t11872 / 4.0 + t11874 / 2.0 + t12235 + t11878 / 2.0 - 3.0 / 2.0 * t11883 - t12238 + t11889 / 2.0 + t11046 - t11052 - t11215;
    (t12227, t12228, t12240)
}
