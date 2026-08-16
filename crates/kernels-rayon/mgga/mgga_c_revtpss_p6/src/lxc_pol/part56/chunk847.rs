//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 847/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk847(t1398: f64, t543: f64, t7910: f64, t7301: f64, t1882: f64, t7274: f64, t2022: f64, t5658: f64, t26054: f64, t5722: f64, t1444: f64, t1883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27845 = t7910 * t1398 * t543;
    let t27846 = t7301 * t27845;
    let t27852 = t7274 * t1882 * t543;
    let t27853 = t7301 * t27852;
    let t27857 = t2022 * t5658 * t543;
    let t27858 = t7301 * t27857;
    let t27861 = t26054 * t5722;
    let t27864 = t1883 * t1444;
    (t27845, t27846, t27852, t27853, t27858, t27861, t27864)
}
