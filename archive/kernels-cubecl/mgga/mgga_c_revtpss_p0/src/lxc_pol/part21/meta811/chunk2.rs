//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2964/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2964<F: Float>(t11710: F, t15614: F, t3091: F, t1063: F, t15937: F, t3172: F, t11656: F, t11672: F, t11675: F, t11927: F, t11991: F, t15596: F, t15965: F, t16128: F, t16140: F, t16152: F, t1675: F, t3117: F, t42580: F, t42606: F, t42904: F, t4786: F, t4831: F, t53885: F) -> F {
    let t53993 = t3091 * t11710 * t15614;
    let t53998 = t1063 * t3172 * t15937;
    let t54013 = -F::cast_from(0.3811023832717309953e-2_f64) * t11672 * t15596 - F::cast_from(0.14291339372689912324e-3_f64) * t42580 + F::cast_from(0.57165357490759649295e-3_f64) * t53993 - F::cast_from(0.85748036236139473944e-3_f64) * t11675 * t15965 + F::cast_from(0.17149607247227894789e-2_f64) * t53998 + F::cast_from(0.45732285992607719436e-2_f64) * t11656 * t16140 + F::cast_from(0.76220476654346199061e-2_f64) * t53885 * t16128 + F::cast_from(0.14291339372689912324e-3_f64) * t42904 * t1675 + F::cast_from(0.42874018118069736972e-3_f64) * t11991 * t4831 + F::cast_from(0.57165357490759649295e-3_f64) * t42606 + F::cast_from(0.25724410870841842183e-2_f64) * t11927 * t3117 * t16152 * t4786;
    t54013
}
