//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1375;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta425(t44951: f64, t5330: f64, t3362: f64, t404: f64, t43766: f64, t13026: f64, t43776: f64, t43813: f64, t3450: f64, t3475: f64, t426: f64, t43816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t44952, t44959, t44974, t45000, t45085, t45106, t45107) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1375(t44951, t5330, t3362, t404, t43766, t13026, t43776, t43813, t3450, t3475, t426, t43816);
    (t44952, t44959, t44974, t45000, t45085, t45106, t45107)
}
