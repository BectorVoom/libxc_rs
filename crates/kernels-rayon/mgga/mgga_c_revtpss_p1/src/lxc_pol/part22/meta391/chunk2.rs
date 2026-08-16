//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1963/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1963(t550: f64, t5658: f64, t543: f64, t3992: f64, t2661: f64, t5610: f64, t9775: f64, t1889: f64, t9779: f64, t828: f64, t9954: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13774 = t550 * t5658;
    let t13775 = t13774 * t543;
    let t13776 = t3992 * t13775;
    let t13778 = 0.14291339372689912324e-4_f64 * t2661 * t13776;
    let t13779 = t9775 * t5610;
    let t13781 = t9779 * t1889;
    let t13783 = t9954 * t828;
    (t13774, t13775, t13776, t13778, t13779, t13781, t13783)
}
