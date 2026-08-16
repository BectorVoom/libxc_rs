//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1538/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1538(t1678: f64, t19462: f64, t1086: f64, t23959: f64, t23997: f64, t3153: f64, t3154: f64, t6299: f64, t12050: f64, t357: f64, t11631: f64, t24042: f64, t359: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80173 = t19462 * t1678;
    let t80243 = t23959 * t1086;
    let t80264 = t23997 * t3153;
    let t80277 = t3154 * t6299;
    let t80350 = t12050 * t357;
    let t80358 = t11631 * t6299;
    let t80396 = t359 * t24042;
    (t80173, t80243, t80264, t80277, t80350, t80358, t80396)
}
