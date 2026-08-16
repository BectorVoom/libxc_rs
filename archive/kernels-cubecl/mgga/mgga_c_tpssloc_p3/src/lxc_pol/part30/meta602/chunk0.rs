//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1991/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1991<F: Float>(t81735: F, t1891: F, t22816: F, t23104: F, t80967: F, t6612: F, t812: F, t836: F, t2690: F, t6619: F, t849: F, t23132: F, t2617: F) -> (F, F, F, F, F, F) {
    let t81736 = F::cast_from(0.69792532988666768264e-2_f64) * t81735;
    let t81742 = t80967 * t1891 * t22816 * t23104;
    let t81743 = F::cast_from(0.43737152435318756759e-3_f64) * t81742;
    let t81749 = t812 * t6612 * t836;
    let t81763 = t812 * t6619 * t2690;
    let t81764 = t81763 * t849;
    let t81769 = t2617 * t23132;
    (t81736, t81743, t81749, t81763, t81764, t81769)
}
