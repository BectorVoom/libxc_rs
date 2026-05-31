//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1195/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1195<F: Float>(t32374: F, t4292: F, t572: F, t26123: F, t7741: F, t28042: F, t7330: F, t1459: F, t34004: F, t2040: F, t28271: F, t127439: F, t127442: F, t127443: F, t127447: F, t127449: F, t127453: F, t127455: F, t127459: F, t1461: F, t1918: F, t32354: F, t32377: F, t33992: F, t5802: F, t8607: F) -> F {
    let t127462 = F::cast_from(6.0_f64) * t572 * t32374 * t4292;
    let t127465 = F::cast_from(12.0_f64) * t572 * t26123 * t7741;
    let t127468 = F::cast_from(12.0_f64) * t572 * t7330 * t28042;
    let t127472 = F::cast_from(6.0_f64) * t1459 * t34004;
    let t127475 = t2040 * t28271;
    let t127477 = F::cast_from(3.0_f64) * t1461 * t33992 + F::cast_from(3.0_f64) * t1918 * t32354 + F::cast_from(6.0_f64) * t5802 * t8607 + F::cast_from(6.0_f64) * t127439 + t127442 + F::cast_from(12.0_f64) * t127443 + t127447 + t127449 + t127453 + t127455 + t127459 + t127462 + t127465 + t127468 + t127472 + F::cast_from(12.0_f64) * t127475 + t32377;
    t127477
}
