//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 360/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk360<F: Float>(t1302: F, t493: F, t511: F, t544: F, t1219: F, t1220: F, t1231: F, t1236: F, t1282: F, t1286: F, t1291: F, t1293: F, t1296: F, t1300: F, t267: F) -> (F, F, F) {
    let t1304 = F::new(4.0) / F::new(15.0) * t493 * t1302;
    let t1306 = F::new(4.0) / F::new(15.0) * t511 * t544;
    let t1307 = t1219 - F::new(4.0) / F::new(45.0) * t1220 - t1231 * t267 / F::new(15.0) - t1236 - t1282 + t1286 + t1291 - t1293 - t1296 + t1300 + t1304 - t1306;
    (t1304, t1306, t1307)
}
