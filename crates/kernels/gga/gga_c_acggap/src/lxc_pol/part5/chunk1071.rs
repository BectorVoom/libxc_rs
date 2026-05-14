//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1071/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1071<F: Float>(t3361: F, t4680: F, t6346: F, t1049: F, t5642: F, t5646: F, t1713: F, t839: F, t3132: F, t345: F, t4099: F, t495: F, t1055: F, t5652: F, t21615: F, t21620: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21663 = t3361 * t4680 * t6346;
    let t21669 = t1049 * t5642;
    let t21671 = t1049 * t5646;
    let t21673 = t1713 * t839;
    let t21675 = t345 * t3132 * t21673;
    let t21677 = t495 * t4099;
    let t21679 = t345 * t1055 * t21677;
    let t21681 = t1049 * t5652;
    let t21684 = t345 * t1055 * t21615;
    let t21687 = t345 * t1055 * t21620;
    (t21663, t21669, t21671, t21673, t21675, t21677, t21679, t21681, t21684, t21687)
}
