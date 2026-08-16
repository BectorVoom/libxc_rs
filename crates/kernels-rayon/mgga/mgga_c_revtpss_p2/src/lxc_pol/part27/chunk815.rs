//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 815/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk815(t125: f64, t3923: f64, t3936: f64, t3938: f64, t3937: f64, t4057: f64, t5673: f64, t1353: f64, t4003: f64, t4056: f64, t2735: f64, t4086: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9826 = t125 * t3923;
    let t9828 = t3936 * t9826 * t3938;
    let t9832 = t5673 * t3937 * t4057;
    let t9835 = t4003 * t1353;
    let t9837 = t3936 * t9826 * t9835;
    let t9840 = t4003 * t4056;
    let t9842 = t5673 * t3937 * t9840;
    let t9845 = t2735 * t4086;
    (t9828, t9832, t9837, t9840, t9842, t9845)
}
