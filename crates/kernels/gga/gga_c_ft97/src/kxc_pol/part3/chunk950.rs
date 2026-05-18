//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 950/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk950<F: Float>(t18675: F, t3864: F, t14127: F, t2567: F, t5064: F, t684: F, t2606: F, t258: F, t4934: F, t10079: F, t18506: F, t9808: F) -> (F, F, F, F) {
    let t18676 = t18675 * t3864;
    let t18677 = t14127 * t18676;
    let t18680 = t2567 * t5064;
    let t18681 = t18680 * t684;
    let t18682 = t2606 * t18681;
    let t18685 = t258 * t4934;
    let t18686 = t18685 * t684;
    let t18687 = t10079 * t18686;
    let t18690 = t9808 * t18506;
    (t18677, t18682, t18687, t18690)
}
