//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 947/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk947(t225: f64, t6625: f64, t6576: f64, t2752: f64, t6665: f64, t10143: f64, t1914: f64, t134: f64, t221: f64, t3034: f64, t371: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23278 = t6625 * t225;
    let t23281 = t6576 * t225;
    let t23290 = t6665 * t2752;
    let t23295 = t1914 * t10143;
    let t23383 = t221 * t134;
    let t23508 = 1.0_f64 / t3034 / t371;
    let t23598 = 1.0_f64 / t3034;
    let t23788 = t2752 * t28;
    (t23278, t23281, t23290, t23295, t23383, t23508, t23598, t23788)
}
