//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1196/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1196(t21490: f64, t32145: f64, t10694: f64, t29439: f64, t24741: f64, t5539: f64, t9647: f64, t10701: f64, t25260: f64, t2558: f64, t10691: f64, t1850: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32147 = 0.51270174867614828558e-2_f64 * t21490 * t32145;
    let t32148 = t29439 * t10694;
    let t32149 = 0.1281754371690370714e-2_f64 * t32148;
    let t32151 = t9647 * t5539 * t24741;
    let t32152 = 0.1281754371690370714e-2_f64 * t32151;
    let t32153 = t29439 * t10701;
    let t32154 = 0.64087718584518535698e-3_f64 * t32153;
    let t32158 = t9647 * t25260 * t2558;
    let t32159 = 0.32043859292259267849e-3_f64 * t32158;
    let t32160 = t1850 * t10691;
    (t32147, t32149, t32152, t32154, t32159, t32160)
}
