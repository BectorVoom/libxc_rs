//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 425/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk425<F: Float>(t1621: F, t226: F, t1236: F, t1282: F, t1286: F, t1291: F, t1293: F, t1296: F, t1608: F, t1611: F, t1612: F, t1615: F, t231: F) -> (F, F) {
    let t1623 = F::new(4.0) / F::new(3.0) * t226 * t1621;
    let t1624 = F::new(8.0) / F::new(3.0) * t1608 + t1611 - t1236 - t1282 + t1286 + t1291 - t1293 - t1296 + F::new(4.0) / F::new(3.0) * t1612 * t231 + F::new(8.0) / F::new(3.0) * t1615 + t1623;
    (t1623, t1624)
}
