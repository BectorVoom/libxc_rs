//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2038/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2038(t2453: f64, t3908: f64, t7275: f64, t1399: f64, t2434: f64, t25880: f64, t25899: f64, t3924: f64, t676: f64, t2022: f64, t9646: f64, t9648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94616 = t2453 * t7275 * t3908;
    let t94633 = t2434 * t1399;
    let t94634 = t25880 * t94633;
    let t94635 = t25899 * t94634;
    let t94639 = t676 * t3924;
    let t94640 = t25880 * t94639;
    let t94641 = t25899 * t94640;
    let t94648 = 0.19637199382202157274e-3_f64 * t9646 * t2022 * t9648;
    (t94616, t94634, t94635, t94640, t94641, t94648)
}
