//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1192/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1192(t23204: f64, t32866: f64, t6562: f64, t1880: f64, t214: f64, t225: f64, t25160: f64, t258: f64, t32809: f64, t6547: f64, t32880: f64, t8335: f64, t87782: f64) -> (f64, f64, f64, f64, f64) {
    let t118885 = t6562 * t23204 * t32866;
    let t118886 = 0.82246703342411321825e-2_f64 * t118885;
    let t118892 = 0.16449340668482264365e-1_f64 * t1880 * t214 * t25160 * t225 * t258;
    let t118893 = t6547 * t32809;
    let t118894 = 0.38381794893125283518e-1_f64 * t118893;
    let t118895 = t32880 * t225;
    let t118901 = 0.16449340668482264365e-1_f64 * t1880 * t87782 * t8335;
    (t118886, t118892, t118894, t118895, t118901)
}
