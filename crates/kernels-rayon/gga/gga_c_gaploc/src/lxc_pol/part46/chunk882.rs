//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 882/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk882(t2487: f64, t41878: f64, t6711: f64, t34600: f64, t544: f64, t9287: f64, t34604: f64, t10532: f64, t10533: f64, t41726: f64, t34400: f64, t34401: f64) -> (f64, f64, f64, f64, f64) {
    let t42363 = t2487 * t6711 * t41878;
    let t42366 = t544 * t34600 * t9287;
    let t42367 = 0.29792074959875355558e-1_f64 * t42366;
    let t42369 = t544 * t34604 * t9287;
    let t42370 = 0.29792074959875355558e-1_f64 * t42369;
    let t42373 = 0.38649669361552115674e3_f64 * t10532 * t10533 * t41726;
    let t42376 = 0.13803453343411469884e3_f64 * t34400 * t34401 * t41726;
    (t42363, t42367, t42370, t42373, t42376)
}
