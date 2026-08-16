//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1068/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1068<F: Float>(t6938: F, t7110: F, t6792: F, t9896: F, t116: F, t2010: F, t2168: F, t21868: F, t22179: F, t22212: F, t22219: F, t22233: F, t22752: F, t22827: F, t22895: F, t23160: F, t23164: F, t23166: F, t23172: F, t23174: F, t3501: F, t686: F, t705: F) -> F {
    let t23188 = t7110 * t6938;
    let t23190 = t9896 * t6792;
    let t23196 = F::cast_from(0.24340852171408521992e2_f64) * t23160 + F::cast_from(0.23439339128023021177e2_f64) * t23164 - F::cast_from(0.81136173904695073307e1_f64) * t23166 - F::cast_from(0.30228422675018518372e-1_f64) * t705 * t22233 + F::cast_from(0.21316635841938984807e2_f64) * t23172 + F::cast_from(0.19472681737126817594e2_f64) * t23174 + F::cast_from(0.10431793787746509426e2_f64) * t686 * t22895 * t116 * t22752 + F::cast_from(0.15647690681619764138e1_f64) * t686 * t2010 * t116 * t21868 + F::cast_from(0.45342634012527777558e0_f64) * t705 * t22827 - F::cast_from(0.18137053605011111023e1_f64) * t2168 * t22179 - F::cast_from(0.33855833396020740576e1_f64) * t23188 - F::cast_from(0.25391875047015555432e1_f64) * t23190 + F::cast_from(0.18137053605011111023e0_f64) * t3501 * t22212 + F::cast_from(0.5441116081503333307e1_f64) * t3501 * t22219;
    t23196
}
