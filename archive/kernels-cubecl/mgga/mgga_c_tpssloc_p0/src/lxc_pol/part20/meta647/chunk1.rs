//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2376/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2376<F: Float>(t14473: F, t2944: F, t10661: F, t1556: F, t10731: F, t14363: F, t300: F, t961: F, t2948: F, t14419: F, t923: F, t10771: F, t1568: F) -> (F, F, F, F, F, F) {
    let t48762 = F::cast_from(0.35089341735807877242e1_f64) * t14473 * t2944;
    let t48763 = t10661 * t1556;
    let t48765 = F::cast_from(0.2894756309764656312e3_f64) * t48763 * t10731;
    let t48766 = t300 * t14363;
    let t48768 = F::cast_from(0.17544670867903938621e1_f64) * t48766 * t961;
    let t48770 = F::cast_from(0.17544670867903938621e1_f64) * t14473 * t2948;
    let t48771 = t14419 * t923;
    let t48776 = t10771 * t1568;
    (t48762, t48765, t48768, t48770, t48771, t48776)
}
