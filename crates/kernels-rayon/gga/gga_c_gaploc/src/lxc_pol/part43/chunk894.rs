//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 894/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk894(t43107: f64, t688: f64, t2508: f64, t779: f64, t1897: f64, t27997: f64, t3009: f64, t7226: f64, t28013: f64, t3276: f64, t8670: f64, t2541: f64, t33680: f64) -> (f64, f64, f64, f64, f64) {
    let t43108 = t43107 * t688;
    let t43111 = 0.76905262301422242837e-2_f64 * t2508 * t779 * t43108;
    let t43115 = 0.46143157380853345701e-1_f64 * t1897 * t7226 * t3009 * t27997;
    let t43119 = 0.92286314761706691402e-1_f64 * t2508 * t7226 * t3009 * t28013;
    let t43122 = 0.53833683610995569986e-1_f64 * t1897 * t3276 * t8670;
    let t43125 = 0.10766736722199113997e0_f64 * t2508 * t2541 * t33680;
    (t43111, t43115, t43119, t43122, t43125)
}
