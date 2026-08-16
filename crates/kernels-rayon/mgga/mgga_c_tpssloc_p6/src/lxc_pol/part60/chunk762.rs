//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 762/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk762(t1268: f64, t28017: f64, t510: f64, t652: f64, t7685: f64, t7756: f64, t5493: f64, t89: f64) -> (f64, f64, f64, f64, f64) {
    let t28019 = 2.0_f64 * t1268 * t28017;
    let t28025 = t510 * t28017;
    let t28027 = 2.0_f64 * t652 * t28025;
    let t28029 = 2.0_f64 * t7685 * t7756;
    let t28030 = t89 * t5493;
    (t28019, t28025, t28027, t28029, t28030)
}
