//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1271/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1271<F: Float>(t3765: F, t6199: F, t8205: F, t851: F, t10182: F, t8170: F, t898: F, t3157: F, t9762: F, t1208: F, t889: F, t22180: F, t27675: F) -> (F, F, F, F, F) {
    let t31052 = F::new(0.1551780387578202009e4) * t6199 * t3765 * t8205 * t851;
    let t31055 = F::new(0.51947577317044391277e2) * t898 * t10182 * t8170;
    let t31057 = F::new(0.17544670867903938621e1) * t9762 * t3157;
    let t31058 = t1208 * t889;
    let t31061 = F::new(0.30762056574649219973e4) * t22180 * t27675 * t31058;
    (t31052, t31055, t31057, t31058, t31061)
}
