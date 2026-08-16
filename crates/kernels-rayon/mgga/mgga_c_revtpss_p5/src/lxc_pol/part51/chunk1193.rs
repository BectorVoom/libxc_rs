//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1193/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1193(t2037: f64, t7956: f64, t1913: f64, t8617: f64, t34015: f64, t571: f64, t2042: f64, t28246: f64, t1916: f64, t32369: f64, t2040: f64, t28277: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127428 = t2037 * t7956;
    let t127434 = t1913 * t8617;
    let t127437 = t571 * t34015;
    let t127439 = t28246 * t2042;
    let t127442 = 12.0_f64 * t1916 * t32369;
    let t127443 = t2040 * t28277;
    (t127428, t127434, t127437, t127439, t127442, t127443)
}
