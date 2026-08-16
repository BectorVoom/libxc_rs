//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta382(t1225: f64, t22671: f64, t1012: f64, t13006: f64, t22688: f64, t13027: f64, t13020: f64, t1774: f64, t6628: f64, t3604: f64, t3720: f64, t3611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24816, t24817, t24820, t24821, t24826, t24827, t24830, t24831, t24834, t24835, t24836, t24839) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1282(t1225, t22671, t1012, t13006, t22688, t13027, t13020, t1774, t6628, t3604, t3720, t3611);
    (t24816, t24817, t24820, t24821, t24826, t24827, t24830, t24831, t24834, t24835, t24836, t24839)
}
