//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1014/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1014<F: Float>(t2752: F, t28: F, t22468: F, t2094: F, t531: F, t7025: F, t9239: F, t33: F, t625: F, t2240: F, t240: F, t67: F) -> (F, F, F, F, F, F, F) {
    let t23788 = t2752 * t28;
    let t23912 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t22468;
    let t23957 = t531 * t2094;
    let t23963 = t9239 * t7025;
    let t23966 = t33 * t625;
    let t23967 = t2240 * t23966;
    let t23992 = t240 * t67;
    (t23788, t23912, t23957, t23963, t23966, t23967, t23992)
}
