//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1023/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1023<F: Float>(t14345: F, t14348: F, t14351: F, t14354: F, t14357: F, t14359: F, t14361: F, t14363: F, t14368: F, t14371: F, t14377: F, t14381: F, t14388: F, t14391: F, t14394: F) -> F {
    let t15132 = F::new(0.80937499999999999999e-1) * t14345 - F::new(0.13489583333333333333e-1) * t14348 - F::new(0.13669444444444444444e1) * t14351 + F::new(0.375e0) * t14354 - F::new(0.60703125e-1) * t14357 + F::new(0.1875e0) * t14359 - F::new(0.40468749999999999999e-1) * t14361 + F::new(0.15e1) * t14363 - F::new(0.5625e0) * t14368 + F::new(0.18333333333333333333e1) * t14371 + F::new(0.29976851851851851851e-2) * t14377 - F::new(0.42777777777777777778e1) * t14381 + F::new(0.25060648148148148148e1) * t14388 - F::new(0.1875e0) * t14391 + F::new(0.10252083333333333334e1) * t14394;
    t15132
}
