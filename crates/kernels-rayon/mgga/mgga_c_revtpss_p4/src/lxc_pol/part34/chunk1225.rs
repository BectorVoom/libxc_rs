//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1225/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1225(t25081: f64, t7897: f64, t198: f64, t206: f64, t7782: f64, t1468: f64, t2411: f64, t11064: f64, t25331: f64, t27216: f64, t1568: f64, t7063: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98450 = t7897 * t25081;
    let t98637 = t198 * t206 * t7782;
    let t98658 = t2411 * t1468;
    let t98722 = t7782 * t11064;
    let t98825 = t27216 * t25331;
    let t98848 = t7063 * t1568;
    (t98450, t98637, t98658, t98722, t98825, t98848)
}
