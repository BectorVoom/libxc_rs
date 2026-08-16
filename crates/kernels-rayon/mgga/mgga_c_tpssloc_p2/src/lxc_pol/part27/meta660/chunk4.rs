//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2309/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2309(t22704: f64, t5336: f64, t80798: f64, t22724: f64, t26436: f64, t81037: f64, t81041: f64, t81043: f64, t81047: f64, t81050: f64, t90865: f64, t90867: f64, t90868: f64, t90873: f64, t90876: f64, t90883: f64, t90887: f64, t90889: f64, t90892: f64, t90895: f64) -> f64 {
    let t90898 = t22704 * t80798 * t5336;
    let t90899 = 0.16449340668482264365e-1_f64 * t90898;
    let t90900 = t22724 * t26436;
    let t90902 = t90865 - t90867 + 0.63969658155208805863e-1_f64 * t90868 - 0.82246703342411321825e-2_f64 * t90873 - 0.19190897446562641759e-1_f64 * t81037 + t90876 + 0.19190897446562641759e-1_f64 * t81041 - 0.11514538467937585055e0_f64 * t81043 - 0.52089578783527170488e-1_f64 * t81047 + 0.82246703342411321824e-2_f64 * t81050 - 0.16449340668482264365e-1_f64 * t90883 - 0.82246703342411321825e-2_f64 * t90887 - t90889 - 0.3289868133696452873e-1_f64 * t90892 + 0.3289868133696452873e-1_f64 * t90895 - t90899 + 0.26044789391763585244e-1_f64 * t90900;
    t90902
}
