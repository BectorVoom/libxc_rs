//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1370;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta422(t13039: f64, t44372: f64, t44373: f64, t13045: f64, t42871: f64, t3597: f64, t3603: f64, t3367: f64, t1209: f64, t13147: f64, t17708: f64, t12854: f64, t17350: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t44441, t44442, t44448, t44449, t44458, t44500, t44510) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1370(t13039, t44372, t44373, t13045, t42871, t3597, t3603, t3367, t1209, t13147, t17708, t12854, t17350);
    (t44441, t44442, t44448, t44449, t44458, t44500, t44510)
}
