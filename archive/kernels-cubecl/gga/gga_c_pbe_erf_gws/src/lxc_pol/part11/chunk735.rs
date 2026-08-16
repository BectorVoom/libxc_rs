//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 735/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk735<F: Float>(t11785: F, t337: F, t2121: F, t2133: F, t3916: F, t3717: F, t5: F, t2147: F, t2164: F, t3832: F, t2142: F, t3783: F) -> (F, F, F, F, F, F, F, F) {
    let t11786 = t337 * t11785;
    let t11787 = t2121 * t11786;
    let t11794 = t3916 * t2133;
    let t11806 = t5 * t3717;
    let t11807 = t337 * t11806;
    let t11808 = t2147 * t11807;
    let t11811 = t2164 * t3832;
    let t11817 = t3783 * t2142;
    (t11786, t11787, t11794, t11806, t11807, t11808, t11811, t11817)
}
