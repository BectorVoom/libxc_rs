//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk991;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta233(t1514: f64, t2289: f64, t1857: f64, t3857: f64, t2516: f64, t5571: f64, t1320: f64, t5569: f64, t2626: f64, t1856: f64, t2608: f64, t512: f64, t2496: f64, t1317: f64, t123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13448, t13584, t13611, t13621, t13630, t13632, t13633) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk991(t1514, t2289, t1857, t3857, t2516, t5571, t1320, t5569, t2626, t1856, t2608, t512);
        let (t13652, t13654, t13665) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk992(t2496, t5571, t1317, t5569, t123, t1856);
    (t13448, t13584, t13611, t13621, t13630, t13632, t13633, t13652, t13654, t13665)
}
