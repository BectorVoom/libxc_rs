//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 241/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk241(t668: f64, t761: f64, t342: f64, t630: f64, t784: f64, t294: f64, t505: f64, t231: f64, t824: f64, t1526: f64, t2320: f64, t343: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2607 = t761 * t668;
    let t2638 = t342 * t630 * t784 / 12.0_f64;
    let t2639 = t294 * t668;
    let t2640 = t2639 * t505;
    let t2644 = t231 * t824;
    let t2648 = t830 - t2638 - t1526 * t2320 * t2640 / 12.0_f64 - t342 * t343 * t2644 / 4.0_f64;
    (t2607, t2638, t2639, t2640, t2644, t2648)
}
