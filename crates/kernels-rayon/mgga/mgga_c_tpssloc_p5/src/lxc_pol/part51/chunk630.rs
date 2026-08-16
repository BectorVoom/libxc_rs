//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 630/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk630(t5010: f64, t5051: f64, t466: f64, t1752: f64, t225: f64, t1251: f64, t1760: f64, t3598: f64, t1243: f64, t5000: f64, t1215: f64, t3612: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5052 = t5010 + t5051;
    let t5053 = t466 * t5052;
    let t5055 = t1752 * t225;
    let t5059 = t1760 * t1251;
    let t5060 = t3598 * t5059;
    let t5064 = t5000 * t1243;
    let t5068 = t3612 * t1215;
    (t5052, t5053, t5055, t5060, t5064, t5068)
}
