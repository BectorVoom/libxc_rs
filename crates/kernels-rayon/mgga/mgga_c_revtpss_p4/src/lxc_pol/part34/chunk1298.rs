//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1298/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1298(t107212: f64, t107636: f64, t1089: f64, t1651: f64, t1668: f64, t1695: f64, t19482: f64, t25464: f64, t25591: f64, t25605: f64, t25629: f64, t27568: f64, t27609: f64, t27640: f64, t27642: f64, t27661: f64, t27699: f64, t29731: f64, t29732: f64, t29744: f64, t29747: f64, t29751: f64, t29875: f64, t29876: f64, t6245: f64, t6259: f64, t6299: f64, t6351: f64, t7145: f64, t7151: f64, t7159: f64, t7817: f64, t7828: f64, t7833: f64, t99721: f64, t99915: f64) -> f64 {
    let t113912 = 0.26020884564615598386e1_f64 * t25605 * t7828 * t6299 * t1089 - 0.26020884564615598386e1_f64 * t27661 * t29876 - 0.26020884564615598386e1_f64 * t25629 * t29875 * t1668 * t1089 - 0.26020884564615598386e1_f64 * t25629 * t7817 * t6299 * t1089 + 0.52041769129231196772e1_f64 * t99915 * t29744 + 0.13010442282307799193e1_f64 * t27640 * t27642 * t19482 * t6299 + 0.26020884564615598386e1_f64 * t27609 * t29732 - 0.78062653693846795158e1_f64 * t7159 * t25464 * t29731 * t1695 + 0.10408353825846239354e2_f64 * t25591 * t7145 * t29747 * t1651 - 0.19756347548806534796e1_f64 * t27568 * t6259 + 0.15612530738769359031e2_f64 * t7151 * t25464 * t29751 * t1651 - 0.13010442282307799193e1_f64 * t107636 * t7833 + 0.39512695097613069591e1_f64 * t99721 * t6245 + 0.39512695097613069591e1_f64 * t27699 * t6351 - 0.26020884564615598386e1_f64 * t107212 * t7833;
    t113912
}
