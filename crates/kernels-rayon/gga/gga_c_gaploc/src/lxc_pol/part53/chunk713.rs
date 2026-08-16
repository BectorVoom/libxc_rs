//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 713/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk713(t12533: f64, t12536: f64, t12065: f64, t895: f64, t11986: f64, t874: f64, t1445: f64, t574: f64, t13728: f64, t597: f64, t12054: f64, t3377: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13795 = 0.38342925953920749677e0_f64 * t12533;
    let t13796 = 0.38342925953920749677e0_f64 * t12536;
    let t13798 = t895 * t12065;
    let t13800 = t11986 * t874;
    let t13801 = t1445 * t13800;
    let t13802 = t574 * t13801;
    let t13805 = t1445 * t13728;
    let t13806 = t597 * t13805;
    let t13808 = t12054 * t3377;
    (t13795, t13796, t13798, t13800, t13801, t13802, t13805, t13806, t13808)
}
