//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1683;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta560(t19049: f64, t6223: f64, t11465: f64, t88008: f64, t973: f64, t981: f64, t23696: f64, t4719: f64, t6227: f64, t300: f64, t88477: f64, t23457: f64, t88264: f64, t964: f64, t2986: f64, t88351: f64, t1642: f64, t78704: f64, t88445: f64, t88448: f64, t88451: f64, t88481: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88580, t88584, t88586, t88588, t88590, t88592) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1683(t19049, t6223, t11465, t88008, t973, t981, t23696, t4719, t6227, t300, t88477, t23457);
        let (t88596, t88600, t88602, t88603) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1684(t88264, t964, t973, t981, t2986, t88351, t1642, t78704, t88445, t88448, t88451, t88481, t88580, t88584, t88586, t88588, t88590, t88592);
    (t88580, t88584, t88586, t88588, t88590, t88592, t88596, t88600, t88602, t88603)
}
