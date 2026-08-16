//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1022/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1022(t1609: f64, t551: f64, t1620: f64, t4536: f64, t4565: f64, t14302: f64, t14305: f64, t14308: f64, t14310: f64, t14313: f64, t14316: f64, t14318: f64, t14322: f64, t14324: f64, t14326: f64, t14328: f64, t14331: f64, t14335: f64, t14338: f64, t14341: f64) -> (f64, f64, f64, f64) {
    let t15092 = t1609 * t1609;
    let t15093 = 1.0_f64 / t15092;
    let t15094 = t551 * t15093;
    let t15095 = t4536 * t1620;
    let t15098 = t1620 * t4565;
    let t15116 = 0.13489583333333333333e-1_f64 * t14302 + 1.0_f64 * t14305 - 0.1875e0_f64 * t14308 - 0.32375000000000000001e0_f64 * t14310 + 0.40468749999999999999e-1_f64 * t14313 - 0.15e1_f64 * t14316 + 0.21583333333333333333e0_f64 * t14318 + 0.5625e0_f64 * t14322 + 0.43166666666666666667e0_f64 * t14324 - 0.625e-1_f64 * t14326 - 0.28125e0_f64 * t14328 - 0.161875e0_f64 * t14331 - 0.9375e-1_f64 * t14335 + 0.32375000000000000001e0_f64 * t14338 + 0.303515625e-1_f64 * t14341;
    (t15094, t15095, t15098, t15116)
}
