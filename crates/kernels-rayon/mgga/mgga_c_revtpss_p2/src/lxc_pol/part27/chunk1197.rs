//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1197/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1197(t2722: f64, t886: f64, t2723: f64, t1032: f64, t2760: f64, t867: f64, t7063: f64, t7060: f64, t136: f64, t2457: f64, t7082: f64, t25299: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92883 = t886 * t2722;
    let t92884 = t92883 * t2723;
    let t92888 = t2760 * t1032;
    let t92889 = t92888 * t867;
    let t92890 = t7063 * t92889;
    let t92891 = t92890 * t7060;
    let t92894 = t7082 * t136 * t2457;
    let t92895 = t25299 * t92894;
    (t92883, t92884, t92888, t92889, t92891, t92894, t92895)
}
