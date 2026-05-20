//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1492/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1492<F: Float>(t116: F, t31451: F, t2212: F, t5789: F, t1513: F, t2: F, t670: F, t8406: F, t1459: F, t1518: F, t1916: F, t21881: F, t2207: F, t22559: F, t22565: F, t22568: F, t31234: F, t31493: F, t31505: F, t31506: F, t31509: F, t31725: F, t31731: F, t31734: F, t4292: F, t572: F, t5802: F, t5920: F, t6941: F, t6945: F, t8336: F, t8342: F, t8346: F, t8421: F) -> (F, F, F) {
    let t118137 = t116 * t31451;
    let t118203 = F::new(2.0) * t5789 * t2212;
    let t118374 = t1513 * t2;
    let t118594 = t670 * t8406;
    let t118629 = F::new(12.0) * t118137 * t1518 * t572 + F::new(12.0) * t118594 * t1518 * t572 + F::new(6.0) * t21881 * t572 * t8342 + F::new(6.0) * t31234 * t572 * t5920 + F::new(6.0) * t31493 * t572 * t5920 + F::new(12.0) * t31505 * t4292 * t572 + F::new(6.0) * t1459 * t31725 + F::new(6.0) * t1459 * t31731 + F::new(3.0) * t1459 * t31734 + F::new(12.0) * t1916 * t31506 + F::new(6.0) * t1916 * t31509 + F::new(12.0) * t2207 * t22559 + F::new(6.0) * t2207 * t22565 + F::new(3.0) * t2207 * t22568 + F::new(12.0) * t5802 * t8421 + F::new(3.0) * t6941 * t8346 + F::new(6.0) * t6945 * t8336;
    (t118203, t118374, t118629)
}
