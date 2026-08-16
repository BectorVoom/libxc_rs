//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1208/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1208(t3487: f64, t486: f64, t7069: f64, t21490: f64, t10694: f64, t29439: f64, t24741: f64, t5539: f64, t9647: f64, t10701: f64, t1897: f64, t32112: f64, t32117: f64, t32119: f64, t32123: f64, t32125: f64, t32128: f64, t32131: f64, t32135: f64, t32139: f64, t32143: f64, t779: f64) -> (f64, f64) {
    let t32145 = t3487 * t486 * t7069;
    let t32147 = 0.51270174867614828558e-2_f64 * t21490 * t32145;
    let t32148 = t29439 * t10694;
    let t32149 = 0.1281754371690370714e-2_f64 * t32148;
    let t32151 = t9647 * t5539 * t24741;
    let t32152 = 0.1281754371690370714e-2_f64 * t32151;
    let t32153 = t29439 * t10701;
    let t32154 = 0.64087718584518535698e-3_f64 * t32153;
    let t32155 = -0.15381052460284448567e-1_f64 * t1897 * t779 * t32112 - t32117 - t32119 - t32123 + t32125 + t32128 - t32131 - t32135 - t32139 - t32143 - t32147 + t32149 + t32152 + t32154;
    (t32145, t32155)
}
