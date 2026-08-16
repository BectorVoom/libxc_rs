//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1194/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1194(t80885: f64, t80899: f64, t80876: f64, t80878: f64, t80889: f64, t80897: f64, t80904: f64, t80906: f64, t80908: f64, t80911: f64, t80915: f64, t80918: f64, t80920: f64, t80922: f64, t80925: f64, t80928: f64, t80931: f64, t80934: f64, t80937: f64, t80940: f64) -> f64 {
    let t84533 = 0.67287926823567318088e-4_f64 * t80885;
    let t84536 = 595.0_f64 / 2592.0_f64 * t80899;
    let t84551 = -t80876 / 64.0_f64 - t80878 / 192.0_f64 - t84533 - 0.35608770875031824732e0_f64 * t80889 - 0.13565246047631171326e0_f64 * t80897 - t84536 - t80904 / 128.0_f64 + t80906 / 128.0_f64 + 5.0_f64 / 64.0_f64 * t80908 - t80911 / 256.0_f64 - 119.0_f64 / 1152.0_f64 * t80915 - 0.12111826828242117256e-2_f64 * t80918 + 0.84782787797694820791e-2_f64 * t80920 + 0.84782787797694820791e-2_f64 * t80922 - 0.40372756094140390853e-3_f64 * t80925 - 0.40372756094140390853e-3_f64 * t80928 + 3.0_f64 / 8.0_f64 * t80931 + 0.50869672678616892474e-1_f64 * t80934 + 0.24223653656484234512e-2_f64 * t80937 - 0.67826230238155856633e-1_f64 * t80940;
    t84551
}
