//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1959;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta562(t2042: f64, t6941: f64, t1916: f64, t7950: f64, t7953: f64, t1936: f64, t5883: f64, t572: f64, t1518: f64, t28276: f64, t5920: f64, t7330: f64, t117: f64, t30004: f64, t1469: f64, t25137: f64, t26776: f64, t29355: f64, t5819: f64, t5825: f64, t5842: f64, t61: f64, t7571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30180, t30182, t30184, t30185, t30187, t30188, t30190, t30191) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1959(t2042, t6941, t1916, t7950, t7953, t1936, t5883, t572, t1518, t28276, t5920, t7330);
        let (t30193, t30194, t30196, t30681) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1960(t30191, t572, t117, t30004, t1469, t25137, t26776, t29355, t5819, t5825, t5842, t61, t7571);
    (t30180, t30182, t30184, t30185, t30187, t30188, t30190, t30191, t30193, t30194, t30196, t30681)
}
