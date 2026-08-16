//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 602/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk602(t8417: f64, t8431: f64, t2347: f64, t8234: f64, t8236: f64, t8238: f64, t8242: f64, t8245: f64, t8249: f64, t8253: f64, t8257: f64, t8261: f64, t8263: f64, t8265: f64, t8269: f64, t8272: f64, t8276: f64, t8280: f64, t8284: f64) -> (f64, f64, f64) {
    let t8432 = t8417 + t8431;
    let t8436 = t2347 * t2347;
    let t8455 = 0.9375e-1_f64 * t8234 - 0.1875e0_f64 * t8236 + 0.125e0_f64 * t8238 + 0.1875e0_f64 * t8242 - 0.125e0_f64 * t8245 - 0.9375e-1_f64 * t8249 - 0.20833333333333333333e-1_f64 * t8253 + 0.625e-1_f64 * t8257 - 0.101171875e-1_f64 * t8261 + 0.20234375e-1_f64 * t8263 - 0.26979166666666666666e-1_f64 * t8265 - 0.20234375e-1_f64 * t8269 + 0.26979166666666666666e-1_f64 * t8272 + 0.101171875e-1_f64 * t8276 - 0.44965277777777777777e-2_f64 * t8280 - 0.13489583333333333333e-1_f64 * t8284;
    (t8432, t8436, t8455)
}
