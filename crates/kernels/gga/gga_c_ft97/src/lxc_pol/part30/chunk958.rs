//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 958/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk958<F: Float>(t1882: F, t33668: F, t33587: F, t5996: F, t1506: F, t6260: F, t1476: F, t6391: F, t7611: F, t880: F, t34312: F, t6213: F) -> (F, F, F, F, F, F) {
    let t142423 = t1882 * t33668;
    let t142434 = t5996 * t33587;
    let t142455 = t6260 * t1506;
    let t142460 = t1476 * t6391;
    let t142485 = t7611 * t880;
    let t142501 = t34312 * t6213;
    (t142423, t142434, t142455, t142460, t142485, t142501)
}
