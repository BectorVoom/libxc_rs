//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2237/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2237(t136: f64, t18499: f64, t18215: f64, t3297: f64, t6014: f64, t699: f64, t1113: f64, t18221: f64, t18225: f64, t6017: f64, t18232: f64, t18237: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18500 = t136 * t18499;
    let t18502 = t3297 * t18215;
    let t18503 = t136 * t18502;
    let t18505 = t699 * t6014;
    let t18507 = t1113 * t18221;
    let t18508 = t136 * t18507;
    let t18509 = t1113 * t18225;
    let t18510 = t136 * t18509;
    let t18512 = t699 * t6017;
    let t18514 = t3297 * t18232;
    let t18515 = t136 * t18514;
    let t18517 = t1113 * t18237;
    (t18500, t18502, t18503, t18505, t18507, t18508, t18509, t18510, t18512, t18514, t18515, t18517)
}
