//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 688/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk688<F: Float>(t369: F, t3772: F, t3912: F, t6216: F, t11459: F, t343: F, t337: F, t2121: F, t2132: F, t3747: F, t1114: F, t11478: F, t2157: F, t2133: F, t3916: F, t3717: F, t5: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11706 = t3772 * t369;
    let t11773 = t3912 * t6216;
    let t11776 = t11459 * t343;
    let t11777 = t337 * t11776;
    let t11778 = t2121 * t11777;
    let t11781 = t3747 * t2132;
    let t11782 = t1114 * t11781;
    let t11785 = t11478 * t2157;
    let t11786 = t337 * t11785;
    let t11787 = t2121 * t11786;
    let t11794 = t3916 * t2133;
    let t11806 = t5 * t3717;
    (t11706, t11773, t11777, t11778, t11781, t11782, t11786, t11787, t11794, t11806)
}
