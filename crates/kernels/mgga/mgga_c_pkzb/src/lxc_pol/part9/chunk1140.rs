//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1140/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1140<F: Float>(t19754: F, t1009: F, t5142: F, t1639: F, t7035: F, t2706: F, t5165: F, t639: F, t7177: F, t1625: F, t2557: F, t83: F) -> (F, F, F, F, F, F) {
    let t19755 = F::new(24.0) * t19754;
    let t19756 = t5142 * t1009;
    let t19757 = F::new(144.0) * t19756;
    let t19758 = t7035 * t1639;
    let t19759 = F::new(0.35089341735807877242e1) * t19758;
    let t19766 = t2706 * t5165;
    let t19770 = t7177 * t639;
    let t19775 = t83 * t2557 * t1625;
    (t19755, t19757, t19759, t19766, t19770, t19775)
}
