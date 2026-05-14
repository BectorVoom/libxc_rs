//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 907/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk907<F: Float>(t1620: F, t4565: F, t14302: F, t14305: F, t14308: F, t14310: F, t14313: F, t14316: F, t14318: F, t14322: F, t14324: F, t14326: F, t14328: F, t14331: F, t14335: F, t14338: F, t14341: F) -> (F, F) {
    let t15098 = t1620 * t4565;
    let t15116 = 0.13489583333333333333e-1 * t14302 + 1.0 * t14305 - 0.1875e0 * t14308 - 0.32375000000000000001e0 * t14310 + 0.40468749999999999999e-1 * t14313 - 0.15e1 * t14316 + 0.21583333333333333333e0 * t14318 + 0.5625e0 * t14322 + 0.43166666666666666667e0 * t14324 - 0.625e-1 * t14326 - 0.28125e0 * t14328 - 0.161875e0 * t14331 - 0.9375e-1 * t14335 + 0.32375000000000000001e0 * t14338 + 0.303515625e-1 * t14341;
    (t15098, t15116)
}
