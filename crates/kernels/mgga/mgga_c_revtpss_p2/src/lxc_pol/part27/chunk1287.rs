//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1287/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1287<F: Float>(t1312: F, t94991: F, t2014: F, t26089: F, t7315: F, t28196: F, t28197: F, t49654: F, t1450: F, t9628: F, t7237: F, t25082: F, t49560: F) -> (F, F, F, F, F) {
    let t94993 = F::cast_from(2.0_f64) * t1312 * t94991;
    let t94998 = F::cast_from(3.0_f64) * t2014 * t26089 * t7315;
    let t95001 = F::cast_from(6.0_f64) * t28196 * t28197 * t49654;
    let t95002 = t1450 * t9628;
    let t95005 = F::cast_from(3.0_f64) * t2014 * t7237 * t95002;
    let t95008 = F::cast_from(18.0_f64) * t25082 * t28197 * t49560;
    (t94993, t94998, t95001, t95005, t95008)
}
