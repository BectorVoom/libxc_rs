//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1116/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1116(t2722: f64, t886: f64, t2723: f64, t1032: f64, t2760: f64, t867: f64, t7063: f64, t1955: f64, t25308: f64, t2769: f64, t10799: f64, t27261: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92883 = t886 * t2722;
    let t92884 = t92883 * t2723;
    let t92888 = t2760 * t1032;
    let t92889 = t92888 * t867;
    let t92890 = t7063 * t92889;
    let t92917 = t1955 * t25308 * t2769;
    let t92942 = t27261 * t10799;
    (t92883, t92884, t92888, t92889, t92890, t92917, t92942)
}
