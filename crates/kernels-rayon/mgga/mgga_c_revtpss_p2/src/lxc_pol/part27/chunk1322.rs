//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1322/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1322(t12657: f64, t2142: f64, t1215: f64, t1248: f64, t12622: f64, t12629: f64, t1287: f64, t1294: f64, t26884: f64, t26891: f64, t26901: f64, t26906: f64, t26918: f64, t26931: f64, t26970: f64, t26984: f64, t26996: f64, t27008: f64, t27020: f64, t3588: f64, t3601: f64, t3739: f64, t3769: f64, t3783: f64, t3791: f64, t7602: f64, t7637: f64, t7651: f64, t7652: f64, t7659: f64, t7666: f64, t97332: f64, t97343: f64, t97348: f64, t97358: f64, t97363: f64) -> f64 {
    let t97370 = t12657 * t2142;
    let t97375 = -0.19756347548806534796e1_f64 * t27008 * t3791 - 0.65854491829355115987e0_f64 * t7602 * t12622 - 0.13010442282307799193e1_f64 * t7659 * t26931 * t3588 * t1287 - 0.26020884564615598386e1_f64 * t26906 * t97332 * t3601 * t3769 + 0.13010442282307799193e1_f64 * t26906 * t26931 * t3601 * t3783 - 0.13010442282307799193e1_f64 * t26918 * t26901 + 0.10408353825846239354e2_f64 * t97343 * t26996 - 0.78062653693846795158e1_f64 * t97348 * t26970 * t1248 * t1287 + 0.26020884564615598386e1_f64 * t7651 * t7652 * t26884 * t1294 + 0.10408353825846239354e2_f64 * t97358 * t7637 * t2142 * t12629 - 0.52041769129231196772e1_f64 * t97363 * t26891 + 0.39512695097613069591e1_f64 * t27008 * t3739 - 0.26020884564615598386e1_f64 * t26984 * t7666 - 0.19756347548806534796e1_f64 * t97370 * t1215 + 0.39512695097613069591e1_f64 * t27020 * t3739;
    t97375
}
