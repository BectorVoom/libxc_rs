//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 772/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk772(t14048: f64, t73935: f64, t12108: f64, t69045: f64, t12111: f64, t69788: f64, t3046: f64, t507: f64, t7262: f64, t12117: f64, t13966: f64, t2046: f64, t8475: f64) -> (f64, f64, f64, f64, f64) {
    let t73936 = t73935 * t14048;
    let t73938 = t69045 * t12108;
    let t73940 = t69788 * t12111;
    let t73943 = t507 * t7262 * t3046;
    let t73944 = t73943 * t12117;
    let t73949 = t2046 * t13966 * t8475;
    (t73936, t73938, t73940, t73944, t73949)
}
