//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1017/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1017(t150327: f64, t3766: f64, t1113: f64, t2344: f64, t27569: f64, t33380: f64, t17836: f64, t24389: f64, t52: f64, t668: f64, t2247: f64, t27511: f64, t33403: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t150358 = t3766 * t150327;
    let t150359 = t2344 * t1113;
    let t150364 = t33380 * t27569;
    let t150367 = t17836 * t24389 * t52;
    let t150372 = t2344 * t668;
    let t150378 = t33403 * t2247 * t27511;
    (t150358, t150359, t150364, t150367, t150372, t150378)
}
