//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1048/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1048<F: Float>(t3912: F, t6216: F, t2138: F, t11459: F, t343: F, t337: F, t2121: F, t2134: F, t2132: F, t3747: F, t1114: F, t11478: F, t2157: F) -> (F, F, F, F, F, F, F) {
    let t11773 = t3912 * t6216;
    let t11775 = t11773 * t2138 / F::cast_from(96.0_f64);
    let t11776 = t11459 * t343;
    let t11777 = t337 * t11776;
    let t11778 = t2121 * t11777;
    let t11780 = t2134 * t11778 / F::cast_from(96.0_f64);
    let t11781 = t3747 * t2132;
    let t11782 = t1114 * t11781;
    let t11784 = t11782 * t2138 / F::cast_from(96.0_f64);
    let t11785 = t11478 * t2157;
    (t11773, t11775, t11776, t11780, t11782, t11784, t11785)
}
