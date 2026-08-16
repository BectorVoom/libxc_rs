//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1244/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1244<F: Float>(t2089: F, t28042: F, t651: F, t2322: F, t34028: F, t4254: F, t1518: F, t32575: F, t28043: F, t7359: F, t34243: F, t7235: F) -> (F, F, F, F, F, F) {
    let t128552 = F::cast_from(2.0_f64) * t651 * t2089 * t28042;
    let t128554 = F::cast_from(2.0_f64) * t2322 * t34028;
    let t128557 = F::cast_from(2.0_f64) * t4254 * t34028;
    let t128560 = F::cast_from(2.0_f64) * t651 * t32575 * t1518;
    let t128562 = F::cast_from(2.0_f64) * t7359 * t28043;
    let t128572 = t7235 * t34243;
    (t128552, t128554, t128557, t128560, t128562, t128572)
}
