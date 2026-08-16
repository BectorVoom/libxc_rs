//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1981/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1981<F: Float>(t81146: F, t22642: F, t22690: F, t22881: F, t154: F, t2690: F, t3748: F, t22691: F, t1887: F, t22797: F) -> (F, F, F, F, F, F) {
    let t81147 = F::cast_from(0.13707783890401886971e-2_f64) * t81146;
    let t81149 = t22642 * t22690 * t22881;
    let t81151 = t2690 * t154;
    let t81152 = t81151 * t3748;
    let t81153 = t81152 * t22691;
    let t81154 = F::cast_from(0.98696044010893586188e-1_f64) * t81153;
    let t81159 = t22797 * t1887;
    (t81147, t81149, t81151, t81152, t81154, t81159)
}
