//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1022/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1022<F: Float>(t1609: F, t551: F, t1620: F, t4536: F, t4565: F, t14302: F, t14305: F, t14308: F, t14310: F, t14313: F, t14316: F, t14318: F, t14322: F, t14324: F, t14326: F, t14328: F, t14331: F, t14335: F, t14338: F, t14341: F) -> (F, F, F, F) {
    let t15092 = t1609 * t1609;
    let t15093 = F::new(1.0) / t15092;
    let t15094 = t551 * t15093;
    let t15095 = t4536 * t1620;
    let t15098 = t1620 * t4565;
    let t15116 = F::cast_from(0.13489583333333333333e-1_f64) * t14302 + F::new(1.0) * t14305 - F::new(0.1875e0) * t14308 - F::cast_from(0.32375000000000000001e0_f64) * t14310 + F::cast_from(0.40468749999999999999e-1_f64) * t14313 - F::new(0.15e1) * t14316 + F::cast_from(0.21583333333333333333e0_f64) * t14318 + F::new(0.5625e0) * t14322 + F::cast_from(0.43166666666666666667e0_f64) * t14324 - F::new(0.625e-1) * t14326 - F::new(0.28125e0) * t14328 - F::new(0.161875e0) * t14331 - F::new(0.9375e-1) * t14335 + F::cast_from(0.32375000000000000001e0_f64) * t14338 + F::cast_from(0.303515625e-1_f64) * t14341;
    (t15094, t15095, t15098, t15116)
}
