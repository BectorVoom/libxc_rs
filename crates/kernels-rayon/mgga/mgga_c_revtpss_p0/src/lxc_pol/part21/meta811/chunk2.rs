//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2964/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2964(t11710: f64, t15614: f64, t3091: f64, t1063: f64, t15937: f64, t3172: f64, t11656: f64, t11672: f64, t11675: f64, t11927: f64, t11991: f64, t15596: f64, t15965: f64, t16128: f64, t16140: f64, t16152: f64, t1675: f64, t3117: f64, t42580: f64, t42606: f64, t42904: f64, t4786: f64, t4831: f64, t53885: f64) -> f64 {
    let t53993 = t3091 * t11710 * t15614;
    let t53998 = t1063 * t3172 * t15937;
    let t54013 = -0.3811023832717309953e-2_f64 * t11672 * t15596 - 0.14291339372689912324e-3_f64 * t42580 + 0.57165357490759649295e-3_f64 * t53993 - 0.85748036236139473944e-3_f64 * t11675 * t15965 + 0.17149607247227894789e-2_f64 * t53998 + 0.45732285992607719436e-2_f64 * t11656 * t16140 + 0.76220476654346199061e-2_f64 * t53885 * t16128 + 0.14291339372689912324e-3_f64 * t42904 * t1675 + 0.42874018118069736972e-3_f64 * t11991 * t4831 + 0.57165357490759649295e-3_f64 * t42606 + 0.25724410870841842183e-2_f64 * t11927 * t3117 * t16152 * t4786;
    t54013
}
