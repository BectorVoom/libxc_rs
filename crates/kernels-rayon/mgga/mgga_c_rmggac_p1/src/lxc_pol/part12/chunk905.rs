//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 905/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk905(t3351: f64, t3352: f64, t5149: f64, t875: f64, t117: f64, t29927: f64, t2295: f64, t16043: f64, t8508: f64, t8808: f64, t302: f64, t35204: f64, t35208: f64, t35212: f64, t35217: f64, t35222: f64, t35226: f64, t35230: f64, t35239: f64, t35242: f64, t35246: f64, t35256: f64, t4965: f64, t72: f64, t8801: f64, t9030: f64) -> f64 {
    let t39635 = t3351 * t3352 * t875 * t5149;
    let t39649 = t29927 * t117;
    let t39650 = t39649 * t2295;
    let t39655 = t16043 * t8508;
    let t39657 = t16043 * t8808;
    let t39659 = 0.51077519871957407277e-4_f64 * t39635 - 0.38422568777328955684e-2_f64 * t35204 + 0.92232789896410962678e-3_f64 * t35208 - 0.10248087766267884742e-3_f64 * t35212 + 0.72042316457491791906e-3_f64 * t35217 - 0.10248087766267884742e-3_f64 * t35222 + 0.60975299583150056628e-3_f64 * t35226 - 0.86737941314158990624e-4_f64 * t35230 + t35239 + 0.60975299583150056628e-3_f64 * t35242 - 0.86737941314158990624e-4_f64 * t35246 - 0.14408463291498358381e-2_f64 * t35256 + 0.79828278012425390428e-1_f64 * t4965 * t8801 - 0.2993560425465952141e-1_f64 * t39650 + 2.0_f64 * t72 * t302 * t9030 - 0.76616279807936110914e-4_f64 * t39655 + 0.10215503974391481455e-3_f64 * t39657;
    t39659
}
