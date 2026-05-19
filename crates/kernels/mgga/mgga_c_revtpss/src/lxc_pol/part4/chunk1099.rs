//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1099/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1099<F: Float>(t114: F, t13513: F, t10416: F, t1312: F, t13425: F, t13426: F, t13429: F, t13435: F, t13440: F, t1518: F, t2322: F, t2371: F, t4248: F, t4292: F, t5523: F, t670: F) -> (F, F) {
    let t115 = F::new(1.0) < t114;
    let t13514 = piecewise3::<F>(t115, F::new(0.0), t13513);
    let t13517 = F::new(2.0) * t10416 * t1518 + F::new(2.0) * t1312 * t13514 + F::new(4.0) * t13426 * t670 + F::new(4.0) * t13435 * t1518 + F::new(2.0) * t13440 * t1518 + F::new(4.0) * t2322 * t4292 + F::new(2.0) * t2371 * t4248 + F::new(4.0) * t4292 * t5523 + t13425 + F::new(2.0) * t13429;
    (t13514, t13517)
}
