//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3185/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3185(t12866: f64, t17514: f64, t56756: f64, t12916: f64, t17723: f64, t3718: f64, t12832: f64, t17617: f64, t12851: f64, t1778: f64, t17429: f64, t17789: f64) -> (f64, f64, f64, f64, f64) {
    let t59078 = t12866 * t56756 * t17514;
    let t59094 = t3718 * t12916 * t17723;
    let t59142 = t12832 * t17617;
    let t59144 = t1778 * t12851;
    let t59146 = t17429 * t17789;
    (t59078, t59094, t59142, t59144, t59146)
}
