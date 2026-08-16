//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 534/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk534(t390: f64, t1070: f64, t193: f64, t2786: f64, t2789: f64, t2796: f64, t2839: f64, t2847: f64, t2937: f64, t2939: f64, t2942: f64, t2946: f64, t2950: f64, t2954: f64, t3209: f64, t3213: f64, t336: f64) -> (f64, f64, f64) {
    let t3215 = t390 * t390;
    let t3216 = 1.0_f64 / t3215;
    let t3219 = t1070 * t193 * t3209 * t336 - t193 * t3213 * t3216 * t336 - t2786 + t2789 - t2796 + t2839 + t2847 + t2937 + t2939 - t2942 + t2946 - t2950 - t2954;
    (t3215, t3216, t3219)
}
