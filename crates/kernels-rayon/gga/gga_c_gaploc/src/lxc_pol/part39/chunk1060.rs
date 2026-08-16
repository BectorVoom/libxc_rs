//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1060/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1060(t33575: f64, t787: f64, t10024: f64, t24549: f64, t7584: f64, t9438: f64, t13064: f64, t825: f64, t826: f64, t10677: f64, t2464: f64, t2465: f64) -> (f64, f64, f64, f64) {
    let t44113 = t787 * t33575;
    let t44114 = t44113 * t10024;
    let t44117 = t7584 * t9438 * t24549;
    let t44118 = 0.15976219147466979032e-1_f64 * t44117;
    let t44120 = t825 * t826 * t13064;
    let t44124 = t825 * t2464 * t2465 * t10677;
    (t44114, t44118, t44120, t44124)
}
