//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 681/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk681<F: Float>(t2735: F, t3780: F, t1701: F, t2719: F, t4109: F, t4088: F, t816: F, t820: F, t4064: F, t2687: F, t283: F, t291: F, t287: F, t4061: F, t1471: F, t800: F) -> (F, F, F, F, F, F, F, F) {
    let t14745 = t3780 * t2735;
    let t14746 = t1701 * t14745;
    let t14749 = t4109 * t2719;
    let t14752 = t816 * t4088;
    let t14753 = t14752 * t820;
    let t14756 = t4064 * t2735;
    let t14759 = t2687 * t283;
    let t14760 = t14759 * t291;
    let t14763 = t4061 * t287;
    let t14766 = t800 * t1471;
    (t14746, t14749, t14753, t14756, t14759, t14760, t14763, t14766)
}
