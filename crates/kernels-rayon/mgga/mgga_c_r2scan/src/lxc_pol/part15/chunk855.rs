//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 855/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk855(t1754: f64, t2788: f64, t2782: f64, t584: f64, t591: f64, t1871: f64, t956: f64, t1859: f64, t970: f64, t5377: f64, t2461: f64, t60: f64) -> (f64, f64, f64, f64, f64) {
    let t7745 = t2788 * t1754;
    let t7751 = 0.1143056e0_f64 * t584 * t2782 * t591;
    let t7753 = t584 * t956 * t1871;
    let t7755 = t1859 * t970;
    let t7756 = t7755 * t5377;
    let t7760 = t60 * t2461;
    (t7745, t7751, t7753, t7756, t7760)
}
