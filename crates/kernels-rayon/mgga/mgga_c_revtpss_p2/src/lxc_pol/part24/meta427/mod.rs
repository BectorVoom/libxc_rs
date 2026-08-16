//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta427(t13180: f64, t493: f64, t225: f64, t13038: f64, t42859: f64, t460: f64, t13045: f64, t43351: f64, t44531: f64, t44535: f64, t1209: f64, t17845: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t45552, t45608, t45610, t45619, t45620, t45654) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1377(t13180, t493, t225, t13038, t42859, t460, t13045, t43351, t44531, t44535, t1209, t17845);
    (t45552, t45608, t45610, t45619, t45620, t45654)
}
