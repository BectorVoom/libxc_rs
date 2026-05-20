//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2894/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2894<F: Float>(t51913: F, t51915: F, t51890: F, t51892: F, t51894: F, t51896: F, t51899: F, t51902: F, t51907: F, t51909: F, t51911: F, t51917: F) -> F {
    let t52546 = F::cast_from(0.69463333333333333334e0_f64) * t51913;
    let t52547 = F::cast_from(0.11577222222222222222e0_f64) * t51915;
    let t52549 = -F::new(0.52945875e1) * t51890 - F::new(0.17648625e1) * t51892 + F::new(0.94674375e0) * t51894 + F::new(0.31558125e0) * t51896 - F::cast_from(0.6618234375e1_f64) * t51899 + F::cast_from(0.2366859375e0_f64) * t51902 - F::new(0.104195e0) * t51907 - F::cast_from(0.83356000000000000001e0_f64) * t51909 + F::cast_from(0.13892666666666666667e0_f64) * t51911 + t52546 - t52547 - F::cast_from(0.41678000000000000001e0_f64) * t51917;
    t52549
}
