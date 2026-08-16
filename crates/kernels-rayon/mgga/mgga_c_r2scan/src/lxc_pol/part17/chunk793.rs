//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 793/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk793(t1871: f64, t584: f64, t956: f64, t1859: f64, t970: f64, t5377: f64, t2461: f64, t60: f64, t170: f64, t1669: f64, t2799: f64, t585: f64) -> (f64, f64, f64, f64, f64) {
    let t7753 = t584 * t956 * t1871;
    let t7755 = t1859 * t970;
    let t7756 = t7755 * t5377;
    let t7760 = t60 * t2461;
    let t7761 = t7760 * t170;
    let t7776 = t2799 * t1669;
    let t7778 = t2461 * t585;
    (t7753, t7756, t7761, t7776, t7778)
}
