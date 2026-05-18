//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 900/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk900<F: Float>(t388: F, t7866: F, t1293: F, t6: F, t368: F, t7240: F, t2: F, t7242: F, t524: F, t7367: F, t23: F, t32075: F) -> (F, F, F, F, F, F) {
    let t36368 = t388 * t7866;
    let t36390 = t1293 * t6;
    let t36450 = F::new(1.0) / t7240 / t368;
    let t36452 = t7242 * t2;
    let t36571 = F::new(1.0) / t7367 / t524;
    let t37429 = t23 * t32075;
    (t36368, t36390, t36450, t36452, t36571, t37429)
}
