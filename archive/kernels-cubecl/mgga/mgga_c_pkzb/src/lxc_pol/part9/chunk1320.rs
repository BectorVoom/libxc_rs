//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1320/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1320<F: Float>(t1167: F, t154: F, t19023: F, t385: F, t3214: F, t6467: F, t1229: F, t17955: F, t918: F, t1238: F, t6428: F, t6476: F, t8319: F) -> (F, F, F, F, F) {
    let t23338 = t385 * t154 * t19023 * t1167;
    let t23340 = t3214 * t6467;
    let t23341 = F::cast_from(0.7622047665434619906e-3_f64) * t23340;
    let t23345 = t918 * t17955 * t1229;
    let t23355 = t1238 * t6428;
    let t23362 = t8319 * t6476;
    (t23338, t23341, t23345, t23355, t23362)
}
