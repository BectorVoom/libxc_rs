//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 825/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk825<F: Float>(t14722: F, t14724: F, t2725: F, t6: F, t285: F, t2726: F, t3780: F, t1701: F, t10363: F, t1208: F, t1196: F, t2724: F, t1200: F, t2735: F, t2719: F, t4109: F) -> (F, F, F, F, F, F, F, F) {
    let t14725 = t14722 * t14724;
    let t14728 = t2725 * t6;
    let t14729 = t285 * t14728;
    let t14730 = t3780 * t2726;
    let t14731 = t1701 * t14730;
    let t14734 = t10363 * t1208;
    let t14738 = t2724 * t1196;
    let t14739 = t14738 * t2726;
    let t14742 = t1200 * t14728;
    let t14745 = t3780 * t2735;
    let t14746 = t1701 * t14745;
    let t14749 = t4109 * t2719;
    (t14725, t14729, t14731, t14734, t14739, t14742, t14746, t14749)
}
