//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1307/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1307(t10828: f64, t2013: f64, t10897: f64, t10896: f64, t1391: f64, t825: f64, t10840: f64, t15362: f64, t10867: f64, t22405: f64, t33378: f64, t969: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33452 = t2013 * t10828;
    let t33453 = 0.38342925953920749676e0_f64 * t33452;
    let t33454 = t2013 * t10897;
    let t33455 = 0.85206502119823888168e-1_f64 * t33454;
    let t33457 = t825 * t1391 * t10896;
    let t33458 = 0.2698205900461089792e0_f64 * t33457;
    let t33459 = t15362 * t10840;
    let t33460 = 0.59584149919750711116e-1_f64 * t33459;
    let t33461 = t10867 * t22405;
    let t33462 = 0.44688112439813033337e-1_f64 * t33461;
    let t33464 = t825 * t969 * t33378;
    (t33453, t33455, t33458, t33460, t33462, t33464)
}
