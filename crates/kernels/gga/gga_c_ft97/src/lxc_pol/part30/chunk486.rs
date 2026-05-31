//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 486/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk486<F: Float>(t2843: F, t7672: F, t296: F, t7644: F, t7648: F, t7652: F, t7656: F, t7660: F) -> (F, F, F) {
    let t7673 = t2843 * t7672;
    let t7674 = t296 * t7673;
    let t7679 = -t7644 + t7648 - t7652 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t7656 - t7660;
    (t7673, t7674, t7679)
}
