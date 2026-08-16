//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1014/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1014<F: Float>(t10261: F, t19709: F, t824: F, t2781: F, t668: F, t5408: F, t505: F, t15047: F, t15042: F, t192: F, t19240: F, t852: F) -> (F, F, F, F, F) {
    let t19711 = t10261 * t19709 * t824;
    let t19714 = t2781 * t668;
    let t19716 = t19714 * t5408 * t824;
    let t19719 = t5408 * t505;
    let t19720 = t15047 * t19719;
    let t19723 = t15042 * t19719;
    let t19727 = t192 * t852 * t19240;
    (t19711, t19716, t19720, t19723, t19727)
}
