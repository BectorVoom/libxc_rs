//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1000/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1000(t10443: f64, t5409: f64, t1882: f64, t5327: f64, t5311: f64, t15402: f64, t18514: f64, t4139: f64, t15386: f64, t15385: f64, t15195: f64, t4261: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19479 = t10443 * t5409;
    let t19482 = t1882 * t5327;
    let t19484 = t1882 * t5311;
    let t19486 = t15402 * t18514;
    let t19487 = t4139 * t19486;
    let t19490 = t15386 * t18514;
    let t19491 = t15385 * t19490;
    let t19494 = t15195 * t4261;
    (t19479, t19482, t19484, t19487, t19491, t19494)
}
