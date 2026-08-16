//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1068/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1068(t6938: f64, t7110: f64, t6792: f64, t9896: f64, t116: f64, t2010: f64, t2168: f64, t21868: f64, t22179: f64, t22212: f64, t22219: f64, t22233: f64, t22752: f64, t22827: f64, t22895: f64, t23160: f64, t23164: f64, t23166: f64, t23172: f64, t23174: f64, t3501: f64, t686: f64, t705: f64) -> f64 {
    let t23188 = t7110 * t6938;
    let t23190 = t9896 * t6792;
    let t23196 = 0.24340852171408521992e2_f64 * t23160 + 0.23439339128023021177e2_f64 * t23164 - 0.81136173904695073307e1_f64 * t23166 - 0.30228422675018518372e-1_f64 * t705 * t22233 + 0.21316635841938984807e2_f64 * t23172 + 0.19472681737126817594e2_f64 * t23174 + 0.10431793787746509426e2_f64 * t686 * t22895 * t116 * t22752 + 0.15647690681619764138e1_f64 * t686 * t2010 * t116 * t21868 + 0.45342634012527777558e0_f64 * t705 * t22827 - 0.18137053605011111023e1_f64 * t2168 * t22179 - 0.33855833396020740576e1_f64 * t23188 - 0.25391875047015555432e1_f64 * t23190 + 0.18137053605011111023e0_f64 * t3501 * t22212 + 0.5441116081503333307e1_f64 * t3501 * t22219;
    t23196
}
