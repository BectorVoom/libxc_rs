//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1045/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1045(t136356: f64, t136403: f64, t136458: f64, t136566: f64, t136666: f64, t136736: f64, t136920: f64, t136926: f64, t136935: f64, t145171: f64, t22532: f64, t22796: f64, t25649: f64, t25653: f64, t25694: f64, t25722: f64, t25771: f64, t25775: f64, t25803: f64, t3030: f64, t32152: f64, t34430: f64, t36364: f64, t36390: f64, t37985: f64, t6427: f64, t7205: f64, t92353: f64) -> f64 {
    let t145297 = 0.89080607335887169333e-3_f64 * t136356 * t34430 - 0.79202200203119310706e-5_f64 * t136666 * t36364 * t25649 + 0.79202200203119310706e-5_f64 * t136926 * t36364 * t25653 - 0.13784064983740990796e-3_f64 * t136736 * t3030 - 0.45497819271775541929e-4_f64 * t136920 * t7205 * t145171 * t25694 - 0.39601100101559655353e-5_f64 * t22796 * t32152 * t25722 - 0.17816121467177433867e-3_f64 * t136566 * t25803 + 0.21120586720831816188e-4_f64 * t136935 * t25771 - 0.59346127734643676855e-4_f64 * t92353 * t36390 * t22532 * t25775 + 0.28200083969358461042e-4_f64 * t136458 - 0.16779431174156321371e-9_f64 * t37985 * t136403 * t6427;
    t145297
}
