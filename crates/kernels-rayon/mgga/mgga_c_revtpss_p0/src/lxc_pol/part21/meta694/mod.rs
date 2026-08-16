//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta694 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta694(t45384: f64, t487: f64, t1269: f64, t3552: f64, t44420: f64, t12690: f64, t44831: f64, t12657: f64, t1204: f64, t3727: f64, t3555: f64, t13180: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45449, t45464, t45482, t45487, t45515, t45522, t45535, t45545, t45551) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2516(t45384, t487, t1269, t3552, t44420, t12690, t44831, t12657, t1204, t3727, t3555, t13180, t493);
    (t45449, t45464, t45482, t45487, t45515, t45522, t45535, t45545, t45551)
}
