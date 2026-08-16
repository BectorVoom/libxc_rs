//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1952/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1952(t1444: f64, t7920: f64, t25924: f64, t1398: f64, t543: f64, t7910: f64, t7301: f64, t1882: f64, t7274: f64, t2022: f64, t5658: f64, t26054: f64, t5722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27840 = t7920 * t1444;
    let t27841 = t25924 * t27840;
    let t27845 = t7910 * t1398 * t543;
    let t27846 = t7301 * t27845;
    let t27852 = t7274 * t1882 * t543;
    let t27853 = t7301 * t27852;
    let t27857 = t2022 * t5658 * t543;
    let t27858 = t7301 * t27857;
    let t27861 = t26054 * t5722;
    (t27840, t27841, t27845, t27846, t27852, t27853, t27857, t27858, t27861)
}
