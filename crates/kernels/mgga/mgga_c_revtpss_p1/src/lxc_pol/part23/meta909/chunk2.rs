//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2921/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2921<F: Float>(t23495: F, t698: F, t52011: F, t52018: F, t77513: F, t41361: F, t51974: F, t51978: F, t63320: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F) -> (F, F, F) {
    let t77736 = t698 * t23495;
    let t77739 = t52011 * t52018 * t77513;
    let t77747 = F::new(0.36231e1) * t77515 - F::cast_from(0.10064166666666666667e1_f64) * t77518 - F::new(0.543465e1) * t77521 + F::new(0.33114e0) * t77736 - F::new(0.149013e1) * t77739 - t51974 + F::cast_from(0.93932222222222222225e0_f64) * t51978 + F::new(0.16557e0) * t63320 + F::cast_from(0.31310740740740740741e0_f64) * t41361 - F::cast_from(0.60384999999999999999e0_f64) * t77527 - F::cast_from(0.60384999999999999999e0_f64) * t77531 + F::new(0.72462e1) * t77535;
    (t77736, t77739, t77747)
}
