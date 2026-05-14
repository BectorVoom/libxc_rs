//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 783/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk783<F: Float>(t6194: F, t684: F, t724: F, t1424: F, t2526: F, t729: F, t762: F, t2469: F, t6088: F, t6061: F, t766: F, t24465: F, t2574: F, t265: F, t242: F, t24408: F) -> (F, F, F, F, F, F, F, F) {
    let t24693 = t724 * t6194 * t684;
    let t24696 = t1424 * t2526;
    let t24698 = t729 * t762 * t24696;
    let t24702 = t729 * t2469 * t6088;
    let t24705 = t6061 * t766;
    let t24707 = t729 * t762 * t24705;
    let t24711 = t2574 * t265 * t24465;
    let t24714 = t242 * t24408;
    (t24693, t24696, t24698, t24702, t24705, t24707, t24711, t24714)
}
