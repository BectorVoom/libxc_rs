//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1858;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta545(t2062: f64, t2769: f64, t786: f64, t26519: f64, t93157: f64, t2453: f64, t2458: f64, t7399: f64, t2070: f64, t41154: f64, t11064: f64, t7427: f64, t25876: f64, t26304: f64, t25894: f64, t2435: f64, t26355: f64, t2097: f64, t22: f64, t25937: f64, t94696: f64, t10115: f64, t2099: f64, t26072: f64, t26292: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95936, t95945, t95948, t95964, t95976) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1858(t2062, t2769, t786, t26519, t93157, t2453, t2458, t7399, t2070, t41154, t11064, t7427);
        let (t96186, t96187, t96197, t96204, t96206, t96210, t96211) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1859(t25876, t26304, t25894, t2435, t26355, t2097, t22, t25937, t94696, t10115, t2099, t26072, t26292);
    (t95936, t95945, t95948, t95964, t95976, t96186, t96187, t96197, t96204, t96206, t96210, t96211)
}
