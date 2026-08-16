//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2425;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta684(t45384: f64, t487: f64, t13180: f64, t493: f64, t225: f64, t13038: f64, t42859: f64, t460: f64, t44531: f64, t1209: f64, t17879: f64, t17845: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45449, t45552, t45607, t45608, t45618, t45619, t45634, t45654) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2425(t45384, t487, t13180, t493, t225, t13038, t42859, t460, t44531, t1209, t17879, t17845);
    (t45449, t45552, t45607, t45608, t45618, t45619, t45634, t45654)
}
