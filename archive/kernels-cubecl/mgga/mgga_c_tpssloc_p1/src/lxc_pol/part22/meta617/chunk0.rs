//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2147/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2147<F: Float>(t52367: F, t3030: F, t4940: F, t3623: F, t11712: F, t11880: F, t491: F, t1734: F, t6739: F, t3609: F, t3242: F, t475: F) -> (F, F, F, F, F, F, F) {
    let t52368 = F::cast_from(0.18518518518518518518e-3_f64) * t52367;
    let t52434 = t4940 * t3030;
    let t52435 = t52434 * t3623;
    let t52479 = t11712 * t11880 * t491;
    let t52480 = t1734 * t6739;
    let t52485 = t52434 * t3609;
    let t52548 = t475 * t3242;
    (t52368, t52434, t52435, t52479, t52480, t52485, t52548)
}
