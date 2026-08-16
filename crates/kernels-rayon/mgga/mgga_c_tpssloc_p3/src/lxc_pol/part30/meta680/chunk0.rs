//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2134/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2134(t28117: f64, t81159: f64, t1377: f64, t6330: f64, t1385: f64, t22635: f64, t26331: f64, t26332: f64, t5187: f64, t19885: f64, t90915: f64, t91004: f64) -> (f64, f64, f64, f64) {
    let t96920 = t81159 * t28117;
    let t96922 = t1377 * t6330;
    let t96925 = t26331 * t22635 * t96922 * t1385;
    let t96929 = t26331 * t22635 * t26332 * t5187;
    let t96935 = t91004 * t90915 * t19885;
    (t96920, t96925, t96929, t96935)
}
