//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1993/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1993(t87835: f64, t87873: f64, t225: f64, t26734: f64, t13072: f64, t13463: f64, t1528: f64, t2054: f64, t218: f64, t259: f64, t26582: f64, t26703: f64, t2713: f64, t47585: f64, t7087: f64, t7107: f64, t85146: f64, t85152: f64, t866: f64, t87893: f64, t92722: f64) -> (f64, f64) {
    let t92910 = 0.3289868133696452873e-1_f64 * t87835;
    let t92938 = 0.3289868133696452873e-1_f64 * t87873;
    let t92939 = t26734 * t225;
    let t92950 = -2.0_f64 * t13463 * t7107 - t47585 * t2054 + 4.0_f64 * t2713 * t26703 + 4.0_f64 * t2713 * t26582 - t92938 - 2.0_f64 * t92939 * t866 - 2.0_f64 * t85146 * t1528 - t85152 * t1528 + 4.0_f64 * t7087 * t13072 + t218 * t92722 * t259 + 0.3289868133696452873e-1_f64 * t87893;
    (t92910, t92950)
}
