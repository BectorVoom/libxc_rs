//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1248/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1248(t37443: f64, t37444: f64, t37448: f64, t37452: f64, t40342: f64, t40346: f64, t42876: f64, t42881: f64, t42885: f64, t42889: f64, t42893: f64, t42897: f64, t42900: f64, t42904: f64, t42908: f64) -> f64 {
    let t43867 = t37443 - t42876 + t40342 - t40346 + t42881 - t42885 - t42889 + t42893 - t42897 + t42900 + 0.30487649791575028314e-3_f64 * t37444 - t37448 + t42904 - t37452 - t42908;
    t43867
}
