//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 704/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk704(t2941: f64, t8313: f64, t2902: f64, t4538: f64, t2942: f64, t2894: f64, t426: f64, t1560: f64, t173: f64, t1559: f64, t1476: f64, t126: f64, t1554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8314 = t2941 * t8313;
    let t8316 = t2902 * t4538;
    let t8317 = t8316 * t2942;
    let t8319 = t426 * t2894;
    let t8321 = t1560 * t173;
    let t8322 = t1559 * t8321;
    let t8324 = t1476 * t2942;
    let t8327 = t126 * t1554;
    (t8314, t8316, t8317, t8319, t8322, t8324, t8327)
}
