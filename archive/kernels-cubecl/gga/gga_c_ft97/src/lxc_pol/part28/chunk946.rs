//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 946/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk946<F: Float>(t32300: F, t66: F, t1616: F, t37481: F, t53: F, t7189: F, t1613: F, t5555: F, t409: F, t5517: F, t1301: F, t136505: F, t32259: F) -> (F, F, F, F, F) {
    let t136735 = t32300 * t66;
    let t136736 = t136735 * t1616;
    let t136740 = t37481 * t53 * t7189;
    let t136759 = t1613 * t5555;
    let t136772 = t5517 * t409;
    let t136807 = t32259 * t1301 * t136505;
    (t136736, t136740, t136759, t136772, t136807)
}
