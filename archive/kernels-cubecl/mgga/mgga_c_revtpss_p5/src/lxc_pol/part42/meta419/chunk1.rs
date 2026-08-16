//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1479/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1479<F: Float>(t1459: F, t1461: F, t1916: F, t1918: F, t2207: F, t2209: F, t31475: F, t31494: F, t31497: F, t31500: F, t31506: F, t31509: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t8336: F, t8343: F, t8346: F, t8421: F, t8427: F, t8430: F) -> F {
    let t31512 = F::cast_from(6.0_f64) * t1459 * t8427 + F::cast_from(3.0_f64) * t1459 * t8430 + F::cast_from(3.0_f64) * t1461 * t8421 + F::cast_from(6.0_f64) * t1916 * t8343 + F::cast_from(3.0_f64) * t1916 * t8346 + F::cast_from(3.0_f64) * t1918 * t8336 + F::cast_from(6.0_f64) * t2207 * t5802 + F::cast_from(3.0_f64) * t2207 * t5805 + F::cast_from(3.0_f64) * t2209 * t5795 + t31475 * t573 + F::cast_from(6.0_f64) * t31494 * t572 + F::cast_from(6.0_f64) * t31497 * t572 + F::cast_from(6.0_f64) * t31500 * t572 + F::cast_from(6.0_f64) * t31506 * t572 + F::cast_from(3.0_f64) * t31509 * t572;
    t31512
}
