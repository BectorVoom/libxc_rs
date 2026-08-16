//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 905/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk905<F: Float>(t3351: F, t3352: F, t5149: F, t875: F, t117: F, t29927: F, t2295: F, t16043: F, t8508: F, t8808: F, t302: F, t35204: F, t35208: F, t35212: F, t35217: F, t35222: F, t35226: F, t35230: F, t35239: F, t35242: F, t35246: F, t35256: F, t4965: F, t72: F, t8801: F, t9030: F) -> F {
    let t39635 = t3351 * t3352 * t875 * t5149;
    let t39649 = t29927 * t117;
    let t39650 = t39649 * t2295;
    let t39655 = t16043 * t8508;
    let t39657 = t16043 * t8808;
    let t39659 = F::cast_from(0.51077519871957407277e-4_f64) * t39635 - F::cast_from(0.38422568777328955684e-2_f64) * t35204 + F::cast_from(0.92232789896410962678e-3_f64) * t35208 - F::cast_from(0.10248087766267884742e-3_f64) * t35212 + F::cast_from(0.72042316457491791906e-3_f64) * t35217 - F::cast_from(0.10248087766267884742e-3_f64) * t35222 + F::cast_from(0.60975299583150056628e-3_f64) * t35226 - F::cast_from(0.86737941314158990624e-4_f64) * t35230 + t35239 + F::cast_from(0.60975299583150056628e-3_f64) * t35242 - F::cast_from(0.86737941314158990624e-4_f64) * t35246 - F::cast_from(0.14408463291498358381e-2_f64) * t35256 + F::cast_from(0.79828278012425390428e-1_f64) * t4965 * t8801 - F::cast_from(0.2993560425465952141e-1_f64) * t39650 + F::cast_from(2.0_f64) * t72 * t302 * t9030 - F::cast_from(0.76616279807936110914e-4_f64) * t39655 + F::cast_from(0.10215503974391481455e-3_f64) * t39657;
    t39659
}
