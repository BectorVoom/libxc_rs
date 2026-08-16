//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2943/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2943(t10236: f64, t17635: f64, t13835: f64, t13847: f64, t2986: f64, t13839: f64, t48279: f64, t17748: f64, t10235: f64, t13851: f64, t4531: f64, t48021: f64, t48024: f64, t48030: f64, t48044: f64, t48048: f64, t48052: f64, t48357: f64) -> f64 {
    let t61279 = t10236 * t17635;
    let t61288 = t2986 * t13847 * t13835;
    let t61291 = t2986 * t48279 * t13839;
    let t61294 = t2986 * t13847 * t17748;
    let t61301 = -0.55555555555555555554e-3_f64 * t2986 * t4531 * t48357 - 0.74074074074074074072e-3_f64 * t2986 * t10235 * t61279 + 0.24691358024691358024e-3_f64 * t48021 + 0.37037037037037037036e-3_f64 * t48024 - 0.37037037037037037036e-3_f64 * t48030 + 0.11111111111111111111e-2_f64 * t48044 + 0.74074074074074074073e-3_f64 * t61288 - 0.49382716049382716048e-3_f64 * t61291 - 0.37037037037037037036e-3_f64 * t61294 + 0.22222222222222222222e-2_f64 * t2986 * t13851 * t13835 - 0.37037037037037037036e-3_f64 * t48048 - 0.18518518518518518518e-3_f64 * t48052;
    t61301
}
