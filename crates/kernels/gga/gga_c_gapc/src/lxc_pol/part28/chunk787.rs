//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 787/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk787<F: Float>(t1081: F, t2752: F, t2685: F, t3357: F, t3360: F, t9857: F, t9860: F, t9866: F, t9869: F, t9872: F, t9874: F, t9876: F, t9878: F, t9881: F, t9883: F, t1: F, t277: F) -> (F, F, F, F, F) {
    let t9885 = t1081 * t2752;
    let t9887 = t3357 * t2685;
    let t9889 = t3360 * t2685;
    let t9891 = -0.17376185052903442709e-3 * t9857 + 0.25745714186718600948e-5 * t9860 + 0.49239311888846044752e-7 * t9866 + 0.17376185052903442709e-3 * t9869 + 0.86880925264517213544e-4 * t9872 + 0.2318836277704281739e-4 * t9874 - 0.15176747947735985782e-6 * t9876 + 0.26984257851074582721e-6 * t9878 - 0.23248749138441366393e-5 * t9881 + 0.21642471925239962898e-3 * t9883 - 0.21642471925239962898e-3 * t9885 - 0.20611878024038059902e-5 * t9887 + 0.36647919126739670507e-5 * t9889;
    let t9894 = t277 * t1;
    (t9885, t9887, t9889, t9891, t9894)
}
