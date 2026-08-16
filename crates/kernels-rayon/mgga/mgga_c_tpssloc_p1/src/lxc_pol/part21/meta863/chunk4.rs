//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3145/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3145(t1174: f64, t44571: f64, t6119: f64, t17686: f64, t44607: f64, t15382: f64, t3447: f64, t52059: f64, t15338: f64, t18542: f64, t15293: f64, t11569: f64, t1177: f64, t15289: f64, t15320: f64, t15376: f64, t3455: f64, t52140: f64, t52281: f64, t52288: f64, t52296: f64, t55723: f64) -> f64 {
    let t65126 = t1174 * t44571 * t6119;
    let t65128 = t44607 * t17686;
    let t65136 = t3447 * t52059 * t15382;
    let t65139 = t3447 * t15338 * t18542;
    let t65142 = t3447 * t15338 * t15293;
    let t65147 = -0.11111111111111111111e-2_f64 * t1174 * t1177 * t3455 * t55723 - 0.2962962962962962963e-2_f64 * t15376 * t15289 + 0.11111111111111111111e-2_f64 * t3447 * t15320 * t18542 - 0.82304526748971193413e-4_f64 * t65126 - 0.44444444444444444442e-2_f64 * t3447 * t11569 * t65128 - 0.14814814814814814814e-2_f64 * t3447 * t52140 * t15382 - 0.49382716049382716048e-3_f64 * t65136 + 0.37037037037037037036e-3_f64 * t65139 + 0.74074074074074074073e-3_f64 * t65142 - 0.20576131687242798353e-3_f64 * t52281 - 0.65843621399176954729e-3_f64 * t52288 - 0.55555555555555555554e-3_f64 * t52296;
    t65147
}
