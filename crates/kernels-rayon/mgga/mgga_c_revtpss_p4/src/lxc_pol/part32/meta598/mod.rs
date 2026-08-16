//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1932;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta598(t2110: f64, t5808: f64, t1455: f64, t8130: f64, t1921: f64, t7541: f64, t28944: f64, t575: f64, t5891: f64, t94978: f64, t665: f64, t94982: f64, t1513: f64, t4287: f64, t25826: f64, t25823: f64, t5915: f64, t21876: f64, t6998: f64, t28166: f64, t7897: f64, t5824: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t104079, t104081, t104083, t104085, t105870, t105873) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1932(t2110, t5808, t1455, t8130, t1921, t7541, t28944, t575, t5891, t94978, t665, t94982);
        let (t105876, t105878, t105881, t105883, t105892, t105898) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1933(t1513, t4287, t25826, t25823, t5915, t665, t21876, t6998, t28166, t7897, t5824, t775);
    (t104079, t104081, t104083, t104085, t105870, t105873, t105876, t105878, t105881, t105883, t105892, t105898)
}
