//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1238/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1238<F: Float>(t2037: F, t8249: F, t1913: F, t8776: F, t34468: F, t575: F, t34490: F, t571: F, t127439: F, t127442: F, t127443: F, t127447: F, t127449: F, t127453: F, t127455: F, t127459: F, t127462: F, t1461: F, t32377: F, t34477: F) -> (F, F, F, F, F) {
    let t129530 = t2037 * t8249;
    let t129531 = t1913 * t8776;
    let t129533 = t34468 * t575;
    let t129534 = t571 * t34490;
    let t129540 = F::cast_from(3.0_f64) * t1461 * t34477 + F::cast_from(3.0_f64) * t127439 + t127442 + F::cast_from(6.0_f64) * t127443 + t127447 + t127449 + t127453 + t127455 + t127459 + t127462 + t32377;
    (t129530, t129531, t129533, t129534, t129540)
}
