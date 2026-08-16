//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2637/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2637(t14587: f64, t2782: f64, t51548: f64, t14602: f64, t14961: f64, t1558: f64, t2482: f64, t4469: f64, t14520: f64, t14568: f64, t14524: f64, t51297: f64) -> (f64, f64, f64, f64, f64) {
    let t62853 = t2782 * t51548 * t14587;
    let t62866 = t2482 * t14961 * t1558 * t14602;
    let t62868 = t4469 * t1558;
    let t62872 = t14568 * t14520;
    let t62874 = t51297 * t14524;
    (t62853, t62866, t62868, t62872, t62874)
}
