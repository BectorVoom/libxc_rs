//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1454/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1454<F: Float>(t50834: F, t71335: F, t71337: F, t77959: F, t77963: F, t77967: F, t77971: F, t77975: F, t77979: F, t77983: F, t77989: F, t77992: F, t77995: F, t77998: F) -> F {
    let t78809 = F::cast_from(0.55570666666666666666e0_f64) * t77959 - F::cast_from(0.10805407407407407407e0_f64) * t77963 - F::cast_from(0.104195e0_f64) * t77967 + F::cast_from(0.62517e0_f64) * t77971 - F::cast_from(0.125034e1_f64) * t77975 + F::cast_from(0.250068e1_f64) * t77979 + F::cast_from(0.104195e0_f64) * t77983 + F::cast_from(0.27785333333333333333e0_f64) * t71335 - F::cast_from(0.166712e1_f64) * t71337 - F::cast_from(0.21424148148148148148e1_f64) * t50834 + F::cast_from(0.123954e2_f64) * t77989 + F::cast_from(0.516475e0_f64) * t77992 - F::cast_from(0.15302962962962962963e1_f64) * t77995 + F::cast_from(0.309885e1_f64) * t77998;
    t78809
}
