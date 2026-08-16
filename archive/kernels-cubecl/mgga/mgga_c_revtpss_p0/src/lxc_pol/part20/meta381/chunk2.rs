//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1385/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1385<F: Float>(t235: F, t4503: F, t2453: F, t10728: F, t9794: F, t10886: F, t40236: F, t808: F, t123: F, t125: F, t2452: F, t40633: F, t810: F) -> (F, F, F) {
    let t40798 = t4503 * t235;
    let t40799 = t2453 * t40798;
    let t40801 = t40799 * t9794 * t10728;
    let t40804 = t10886 * t808 * t40236;
    let t40810 = F::cast_from(0.30119321664969771194e-5_f64) * t123 * t125 * t40633 * t2452 * t810;
    (t40801, t40804, t40810)
}
