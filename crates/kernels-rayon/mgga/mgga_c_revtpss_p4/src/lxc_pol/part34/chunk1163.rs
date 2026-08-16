//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1163/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1163(t1651: f64, t7817: f64, t7145: f64, t25672: f64, t3304: f64, t6305: f64, t3318: f64, t7168: f64, t1695: f64, t7160: f64, t1976: f64, t6244: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29843 = t7817 * t1651;
    let t29844 = t7145 * t29843;
    let t29848 = t25672 * t6305 * t3304;
    let t29852 = t7168 * t6305 * t3318;
    let t29865 = t7817 * t1695;
    let t29866 = t7160 * t29865;
    let t29871 = t1976 * t6244;
    (t29843, t29844, t29848, t29852, t29866, t29871)
}
