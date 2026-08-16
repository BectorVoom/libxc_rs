//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1296/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1296(t2142: f64, t6564: f64, t30840: f64, t460: f64, t1769: f64, t1828: f64, t1032: f64, t6695: f64, t2148: f64, t1209: f64, t30882: f64, t7658: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t112706 = t6564 * t2142;
    let t112714 = t460 * t30840;
    let t112721 = t1769 * t1828;
    let t112757 = t6695 * t1032;
    let t112758 = t2148 * t112757;
    let t112774 = t1209 * t112757;
    let t112843 = t30882 * t7658;
    (t112706, t112714, t112721, t112757, t112758, t112774, t112843)
}
