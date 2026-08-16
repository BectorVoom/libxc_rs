//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1837/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1837<F: Float>(t24815: F, t3611: F, t24814: F, t1209: F, t24813: F, t1011: F, t475: F, t1193: F, t7372: F) -> (F, F, F, F, F, F, F) {
    let t24816 = t3611 * t24815;
    let t24817 = t24814 * t24816;
    let t24820 = t24813 * t1209;
    let t24821 = t1011 * t475;
    let t24822 = t3611 * t24821;
    let t24823 = t24820 * t24822;
    let t24826 = t7372 * t1193;
    (t24816, t24817, t24820, t24821, t24822, t24823, t24826)
}
