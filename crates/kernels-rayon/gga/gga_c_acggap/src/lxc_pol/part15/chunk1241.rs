//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1241/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1241(t35307: f64, t35352: f64, t35353: f64, t35359: f64, t35361: f64, t37476: f64, t37479: f64, t37484: f64, t37485: f64, t37498: f64, t39876: f64, t39879: f64, t39883: f64, t39885: f64, t39889: f64, t39893: f64, t39897: f64, t39899: f64) -> f64 {
    let t41843 = t37476 + t37479 - 0.51448821741683684367e-2_f64 * t35307 - 0.37737710747524982482e-2_f64 * t39876 + 0.85748036236139473944e-3_f64 * t39879 - t37484 - t37485 + 0.42874018118069736972e-3_f64 * t39883 + 0.56606566121287473724e-1_f64 * t39885 - 0.51448821741683684368e-2_f64 * t39889 + 0.94344276868812456206e-2_f64 * t39893 - 0.62896184579208304137e-2_f64 * t39897 - 0.25724410870841842183e-2_f64 * t39899 - t37498 + t35352 - t35353 + 0.7844375e0_f64 * t35359 + t35361;
    t41843
}
