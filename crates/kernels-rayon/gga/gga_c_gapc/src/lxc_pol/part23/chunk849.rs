//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 849/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk849(t2660: f64, t2767: f64, t8639: f64, t1081: f64, t2807: f64, t2752: f64, t2685: f64, t3357: f64, t3360: f64, t9857: f64, t9860: f64, t9866: f64, t9869: f64, t9872: f64, t9874: f64, t9876: f64, t9878: f64) -> f64 {
    let t9881 = t2660 * t8639 * t2767;
    let t9883 = t1081 * t2807;
    let t9885 = t1081 * t2752;
    let t9887 = t3357 * t2685;
    let t9889 = t3360 * t2685;
    let t9891 = -0.17376185052903442709e-3_f64 * t9857 + 0.25745714186718600948e-5_f64 * t9860 + 0.49239311888846044752e-7_f64 * t9866 + 0.17376185052903442709e-3_f64 * t9869 + 0.86880925264517213544e-4_f64 * t9872 + 0.2318836277704281739e-4_f64 * t9874 - 0.15176747947735985782e-6_f64 * t9876 + 0.26984257851074582721e-6_f64 * t9878 - 0.23248749138441366393e-5_f64 * t9881 + 0.21642471925239962898e-3_f64 * t9883 - 0.21642471925239962898e-3_f64 * t9885 - 0.20611878024038059902e-5_f64 * t9887 + 0.36647919126739670507e-5_f64 * t9889;
    t9891
}
