//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1164/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1164<F: Float>(t3262: F, t3276: F, t42919: F, t2995: F, t3250: F, t3424: F, t3461: F, t42871: F, t42874: F, t42876: F, t42881: F, t42885: F, t42889: F, t42893: F, t42897: F, t42900: F, t42904: F, t42908: F, t42911: F, t42914: F, t42918: F) -> (F, F) {
    let t42922 = F::new(15.0) / F::new(8.0) * t3262 * t3276 * t42919;
    let t42924 = t2995 * t3461 + t3250 * t3424 + t42871 - t42874 + t42876 - t42881 + t42885 + t42889 - t42893 + t42897 - t42900 - t42904 + t42908 + t42911 - t42914 + t42918 - t42922;
    (t42922, t42924)
}
