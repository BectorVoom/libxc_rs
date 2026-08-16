//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2121/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2121(t10403: f64, t10404: f64, t10422: f64, t10477: f64, t67: f64, t3067: f64, t11059: f64, t10970: f64, t820: f64, t10418: f64, t3070: f64, t10397: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42380 = t10403 * t10422 * t10404;
    let t42386 = t10477 * t67;
    let t42387 = t3067 * t42386;
    let t42388 = t11059 * t42387;
    let t42397 = t820 * t10970;
    let t42403 = t3070 * t10422 * t10418;
    let t42412 = t3070 * t10422 * t10397;
    (t42380, t42386, t42387, t42388, t42397, t42403, t42412)
}
