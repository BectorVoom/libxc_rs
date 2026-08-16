//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1576/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1576(t1269: f64, t1284: f64, t1209: f64, t1204: f64, t3781: f64, t5462: f64) -> (f64, f64, f64, f64) {
    let t12722 = t1284 * t1269;
    let t12723 = t1209 * t12722;
    let t12744 = t1204 * t3781;
    let t12751 = t1209 * t5462;
    (t12722, t12723, t12744, t12751)
}
