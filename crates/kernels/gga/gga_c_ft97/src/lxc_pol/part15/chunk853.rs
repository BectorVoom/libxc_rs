//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 853/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk853<F: Float>(t240: F, t7513: F, t294: F, t7639: F, t1107: F, t5011: F, t13: F, t21: F, t2: F, t7242: F, t113: F, t10: F, t11175: F, t83: F) -> (F, F, F, F, F, F, F) {
    let t33300 = F::new(1.0) / t7513 / t240;
    let t33828 = F::new(1.0) / t7639 / t294;
    let t35382 = t5011 * t1107;
    let t36377 = t13 * t21;
    let t36452 = t7242 * t2;
    let t36827 = t13 * t113;
    let t37292 = t10 * t11175 * t83;
    (t33300, t33828, t35382, t36377, t36452, t36827, t37292)
}
