//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 509/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk509<F: Float>(t1318: F, t2532: F, t2263: F, t2463: F, t2470: F, t2475: F, t2477: F, t2482: F, t2501: F, t2503: F, t2507: F, t2509: F, t2511: F, t2530: F, t256: F) -> (F, F) {
    let t2534 = F::new(8.0) / F::new(15.0) * t1318 * t2532;
    let t2535 = F::new(4.0) / F::new(9.0) * t2263 + t2463 * t256 / F::new(3.0) + t2470 + t2475 + t2477 + t2482 - t2501 + t2503 + t2507 - t2509 - t2511 - t2530 - t2534;
    (t2534, t2535)
}
