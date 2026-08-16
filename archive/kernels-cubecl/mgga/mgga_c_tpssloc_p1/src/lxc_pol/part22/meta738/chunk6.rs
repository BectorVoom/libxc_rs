//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2428/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2428<F: Float>(t59759: F, t59761: F, t60308: F, t60310: F, t60312: F, t68638: F, t68640: F, t68643: F, t68646: F, t68649: F, t68695: F, t68697: F) -> F {
    let t69156 = F::cast_from(0.94674375e0_f64) * t68638 + F::cast_from(0.94674375e0_f64) * t68640 - F::cast_from(0.17648625e1_f64) * t68643 + F::cast_from(0.31558125e0_f64) * t68646 - F::cast_from(0.104195e0_f64) * t68649 + F::cast_from(0.309885e1_f64) * t59759 - F::cast_from(0.20659e1_f64) * t59761 - F::cast_from(0.41678000000000000001e0_f64) * t60308 + F::cast_from(0.13892666666666666667e0_f64) * t60310 + F::cast_from(0.9261777777777777778e-1_f64) * t60312 + F::cast_from(0.3529725e1_f64) * t68695 + F::cast_from(0.6311625e0_f64) * t68697;
    t69156
}
