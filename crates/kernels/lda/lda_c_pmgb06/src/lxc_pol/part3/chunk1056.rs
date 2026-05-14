//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1056/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1056<F: Float>(t13315: F, t13318: F, t13322: F, t13323: F, t13324: F, t13327: F, t13328: F, t13421: F, t13423: F, t13425: F, t13427: F, t10720: F, t10727: F, t10732: F, t10735: F, t13429: F, t13431: F, t13433: F, t13435: F, t13438: F, t13440: F, t13453: F, t13455: F) -> (F, F) {
    let t14424 = t13315 - t13318 + t13322 - t13323 - t13324 + t13327 - t13328 + t13421 + t13423 + t13425 + t13427;
    let t14428 = 4.0 * t10720 + t10727 + 4.0 / 3.0 * t10732 + 4.0 * t10735 - t13429 - t13431 + t13433 - t13435 - t13438 + t13440 + t13453 - t13455;
    (t14424, t14428)
}
