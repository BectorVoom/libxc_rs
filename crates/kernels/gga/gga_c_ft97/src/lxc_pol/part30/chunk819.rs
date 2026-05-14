//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 819/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk819<F: Float>(t22511: F, t28658: F, t7003: F, t28676: F, t2691: F, t28557: F, t4113: F, t28719: F, t317: F, t2842: F, t7091: F, t2766: F, t6353: F, t10491: F, t1508: F, t10478: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t111837 = t28658 * t22511;
    let t111838 = t7003 * t111837;
    let t112071 = t28676 * t111837;
    let t112156 = t2691 * t28557;
    let t112159 = t4113 * t111837;
    let t112268 = t2691 * t111837;
    let t112384 = t28719 * t317;
    let t112390 = t7091 * t2842;
    let t112663 = t2766 * t6353;
    let t112680 = t10491 * t1508;
    let t112746 = t10478 * t1508;
    (t111838, t112071, t112156, t112159, t112268, t112384, t112390, t112663, t112680, t112746)
}
