//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1707/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1707(t5614: f64, t6614: f64, t5617: f64, t815: f64, t6605: f64, t2628: f64, t5585: f64, t23146: f64, t5593: f64, t1894: f64, t236: f64, t5544: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28370 = t6614 * t5614;
    let t28372 = t815 * t5617;
    let t28373 = t6605 * t28372;
    let t28375 = t2628 * t5585;
    let t28376 = t6605 * t28375;
    let t28380 = t23146 * t5593;
    let t28383 = t1894 * t236 * t5544;
    (t28370, t28372, t28373, t28375, t28376, t28380, t28383)
}
