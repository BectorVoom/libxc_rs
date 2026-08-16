//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 478/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk478<F: Float>(t13848: F, t13850: F, t1977: F, t2190: F, t3148: F, t3151: F, t2191: F, t3154: F, t1986: F, t2125: F, t675: F, t1004: F, t7: F) -> (F, F, F, F, F, F) {
    let t13851 = t1977 * t13848 * t13850;
    let t13854 = t2190 * t3148 * t3151;
    let t13856 = t2191 * t3154;
    let t13858 = t1986 * t2125;
    let t13859 = t675 * t13858;
    let t13861 = t7 * t1004;
    (t13851, t13854, t13856, t13858, t13859, t13861)
}
