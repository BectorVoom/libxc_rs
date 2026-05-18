//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 864/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk864<F: Float>(t2545: F, t7200: F, t10008: F, t320: F, t9029: F, t315: F, t7216: F, t2664: F, t9501: F, t2316: F, t2636: F, t3378: F) -> (F, F, F, F) {
    let t10009 = t2545 * t7200;
    let t10010 = t10008 * t10009;
    let t10012 = t320 * t9029;
    let t10013 = t315 * t7216;
    let t10014 = t10012 * t10013;
    let t10016 = t9501 * t2664;
    let t10018 = t2636 * t2316;
    let t10019 = t3378 * t10018;
    (t10010, t10014, t10016, t10019)
}
