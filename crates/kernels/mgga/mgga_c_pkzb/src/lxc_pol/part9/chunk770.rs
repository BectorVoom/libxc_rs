//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 770/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk770<F: Float>(t5122: F, t5160: F, t5195: F, t5470: F, t45: F, t292: F, t197: F, t323: F, t1824: F, t815: F, t2168: F, t645: F) -> (F, F, F, F, F, F) {
    let t5472 = t5122 + t5160 + t5195 + t5470;
    let t5473 = t45 * t5472;
    let t5474 = F::cast_from(1.0_f64) / t292;
    let t5475 = t197 * t5474;
    let t5476 = t5475 * t323;
    let t5477 = F::cast_from(0.57375e0_f64) * t5476;
    let t5478 = t1824 * t815;
    let t5479 = F::cast_from(0.4303125e0_f64) * t5478;
    let t5480 = t645 * t2168;
    (t5472, t5473, t5475, t5477, t5479, t5480)
}
