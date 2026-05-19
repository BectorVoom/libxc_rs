//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 710/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk710<F: Float>(t6772: F, t104: F, t188: F, t6465: F, t6741: F, t6744: F, t6747: F, t6750: F, t6753: F, t6757: F, t6761: F, t6763: F, t6766: F, t6771: F, t95: F) -> (F, F) {
    let t6773 = F::new(60.0) * t6772;
    let t6774 = t6741 + t6744 - t6747 - t6750 + t6753 + t188 * t6757 / F::new(2.0) - F::new(7.0) / F::new(2.0) * t6761 + t6465 + F::cast_from(0.51689762869806860992e-2_f64) * t95 * t104 * t6763 * t6766 + t6771 + t6773;
    (t6773, t6774)
}
