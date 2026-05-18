//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 855/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk855<F: Float>(t1456: F, t6837: F, t729: F, t242: F, t35302: F, t1175: F, t2574: F, t7440: F, t1449: F, t762: F, t13927: F, t7546: F) -> (F, F, F, F, F, F) {
    let t35714 = t729 * t1456 * t6837;
    let t35717 = t242 * t35302;
    let t35721 = t2574 * t1175 * t7440;
    let t35724 = t6837 * t1449;
    let t35726 = t729 * t762 * t35724;
    let t35729 = t13927 * t7546;
    (t35714, t35717, t35721, t35724, t35726, t35729)
}
