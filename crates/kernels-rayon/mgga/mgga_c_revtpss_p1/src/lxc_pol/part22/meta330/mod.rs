//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1785;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta330(t2645: f64, t2723: f64, t10115: f64, t253: f64, t10867: f64, t251: f64, t233: f64, t2760: f64, t869: f64, t689: f64, t2777: f64, t2789: f64, t2439: f64, t2435: f64, t2790: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t10943 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1785(t2645, t2723);
        let (t10948, t10952, t10959, t10960, t10961, t10963, t10964, t10966) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1786(t10115, t253, t10867, t251, t233, t2760, t869, t689, t2777, t2789, t2439, t2435, t2790);
    (t10943, t10948, t10952, t10959, t10960, t10961, t10963, t10964, t10966)
}
