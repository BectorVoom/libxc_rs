//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 956/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk956<F: Float>(t32063: F, t32078: F, t7238: F, t1317: F, t1637: F, t7248: F, t1636: F, t7256: F, t89: F, t7260: F, t32360: F, t375: F) -> (F, F, F, F, F, F, F, F) {
    let t137180 = t7238 * t32063 * t32078;
    let t137197 = t1317 * t1637 * t7248;
    let t137198 = F::new(4.0) / F::new(27.0) * t137197;
    let t137204 = t89 * t1636 * t7256;
    let t137205 = F::new(8.0) / F::new(27.0) * t137204;
    let t137212 = t89 * t1636 * t7260;
    let t137213 = F::new(4.0) / F::new(27.0) * t137212;
    let t137215 = t89 * t375 * t32360;
    (t137180, t137197, t137198, t137204, t137205, t137212, t137213, t137215)
}
