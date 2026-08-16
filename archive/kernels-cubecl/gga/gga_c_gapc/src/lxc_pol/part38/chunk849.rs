//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 849/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk849<F: Float>(t2660: F, t2767: F, t8639: F, t1081: F, t2807: F, t2752: F, t2685: F, t3357: F, t3360: F, t9857: F, t9860: F, t9866: F, t9869: F, t9872: F, t9874: F, t9876: F, t9878: F) -> F {
    let t9881 = t2660 * t8639 * t2767;
    let t9883 = t1081 * t2807;
    let t9885 = t1081 * t2752;
    let t9887 = t3357 * t2685;
    let t9889 = t3360 * t2685;
    let t9891 = -F::cast_from(0.17376185052903442709e-3_f64) * t9857 + F::cast_from(0.25745714186718600948e-5_f64) * t9860 + F::cast_from(0.49239311888846044752e-7_f64) * t9866 + F::cast_from(0.17376185052903442709e-3_f64) * t9869 + F::cast_from(0.86880925264517213544e-4_f64) * t9872 + F::cast_from(0.2318836277704281739e-4_f64) * t9874 - F::cast_from(0.15176747947735985782e-6_f64) * t9876 + F::cast_from(0.26984257851074582721e-6_f64) * t9878 - F::cast_from(0.23248749138441366393e-5_f64) * t9881 + F::cast_from(0.21642471925239962898e-3_f64) * t9883 - F::cast_from(0.21642471925239962898e-3_f64) * t9885 - F::cast_from(0.20611878024038059902e-5_f64) * t9887 + F::cast_from(0.36647919126739670507e-5_f64) * t9889;
    t9891
}
