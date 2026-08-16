//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 884/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk884(t44771: f64, t701: f64, t11603: f64, t1897: f64, t7068: f64, t7226: f64, t3650: f64, t7671: f64, t2508: f64, t2936: f64, t33561: f64, t13548: f64, t2549: f64) -> (f64, f64, f64, f64, f64) {
    let t45091 = t44771 * t701;
    let t45101 = 0.46143157380853345701e-1_f64 * t1897 * t7226 * t11603 * t7068;
    let t45104 = 0.53833683610995569986e-1_f64 * t1897 * t3650 * t7671;
    let t45107 = 0.10766736722199113997e0_f64 * t2508 * t2936 * t33561;
    let t45108 = t2549 * t13548;
    (t45091, t45101, t45104, t45107, t45108)
}
