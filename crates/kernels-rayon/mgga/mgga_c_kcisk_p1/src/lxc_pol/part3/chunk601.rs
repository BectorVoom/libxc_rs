//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 601/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk601(t5082: f64, t1060: f64, t696: f64, t213: f64, t695: f64, t1849: f64, t967: f64, t158: f64, t165: f64, t173: f64, t1809: f64, t3293: f64, t5111: f64, t5114: f64, t5117: f64, t5122: f64, t5125: f64, t5128: f64) -> (f64, f64) {
    let t5129 = 0.23911438650126355246e-1_f64 * t5082;
    let t5130 = t696 * t1060;
    let t5134 = t213 * t695;
    let t5135 = 0.15538616723388920628e-3_f64 * t5134;
    let t5136 = t967 * t1849;
    let t5137 = t5136 * t1060;
    let t5139 = 0.7026e-2_f64 * t158 * t5111 - 0.1585e-2_f64 * t165 * t5114 - 0.10082625e-4_f64 * t173 * t5117 + t5122 - t5125 - t5128 - t5129 - 0.23911438650126355246e-1_f64 * t5130 + 0.11955719325063177623e-1_f64 * t1809 * t3293 + t5135 + 0.20718155631185227504e-3_f64 * t5137;
    (t5136, t5139)
}
