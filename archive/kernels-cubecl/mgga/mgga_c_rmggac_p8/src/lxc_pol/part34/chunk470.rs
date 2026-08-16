//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 470/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk470<F: Float>(t262: F, t321: F, t3068: F, t7282: F, t333: F, t12200: F, t2084: F, t664: F, t27: F, t2145: F, t2020: F, t3061: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13797 = t262 * t321;
    let t13798 = t3068 * t13797;
    let t13799 = t7282 * t13798;
    let t13801 = t262 * t333;
    let t13802 = t3068 * t13801;
    let t13803 = t12200 * t13802;
    let t13805 = t2084 * t664;
    let t13806 = t27 * t13805;
    let t13807 = t2145 * t13806;
    let t13809 = t2020 * t3061;
    (t13797, t13798, t13799, t13801, t13802, t13803, t13806, t13807, t13809)
}
