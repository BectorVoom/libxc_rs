//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1241/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1241<F: Float>(t35307: F, t35352: F, t35353: F, t35359: F, t35361: F, t37476: F, t37479: F, t37484: F, t37485: F, t37498: F, t39876: F, t39879: F, t39883: F, t39885: F, t39889: F, t39893: F, t39897: F, t39899: F) -> F {
    let t41843 = t37476 + t37479 - F::cast_from(0.51448821741683684367e-2_f64) * t35307 - F::cast_from(0.37737710747524982482e-2_f64) * t39876 + F::cast_from(0.85748036236139473944e-3_f64) * t39879 - t37484 - t37485 + F::cast_from(0.42874018118069736972e-3_f64) * t39883 + F::cast_from(0.56606566121287473724e-1_f64) * t39885 - F::cast_from(0.51448821741683684368e-2_f64) * t39889 + F::cast_from(0.94344276868812456206e-2_f64) * t39893 - F::cast_from(0.62896184579208304137e-2_f64) * t39897 - F::cast_from(0.25724410870841842183e-2_f64) * t39899 - t37498 + t35352 - t35353 + F::cast_from(0.7844375e0_f64) * t35359 + t35361;
    t41843
}
