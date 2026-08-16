//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1396/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1396(t1992: f64, t550: f64, t6976: f64, t74941: f64, t22897: f64, t3792: f64, t1336: f64, t1825: f64, t19815: f64, t20473: f64, t20490: f64, t20495: f64, t20554: f64, t22709: f64, t26403: f64, t26458: f64, t28174: f64, t5234: f64, t5334: f64, t6378: f64, t6415: f64, t6987: f64, t7745: f64, t7747: f64, t81243: f64, t90807: f64, t90837: f64, t90868: f64, t90900: f64, t96937: f64, t96945: f64, t96989: f64, t97193: f64) -> f64 {
    let t107281 = t1992 * t6976 * t74941 * t550;
    let t107303 = t1992 * t22897 * t74941 * t3792;
    let t107314 = -0.11514538467937585055e0_f64 * t96937 - 0.38381794893125283518e0_f64 * t90807 + 0.57572692339687925277e-1_f64 * t96945 - 0.24674011002723396548e-1_f64 * t107281 + 6.0_f64 * t5334 * t26403 * t20473 - 0.15626873635058151147e0_f64 * t90837 - 6.0_f64 * t1336 * t81243 * t20490 + 6.0_f64 * t1336 * t22709 * t20495 - 3.0_f64 * t1336 * t97193 * t1825 + 0.19190897446562641759e0_f64 * t90868 - t1336 * t6987 * t20554 - 3.0_f64 * t19815 * t7745 + 0.49348022005446793095e-1_f64 * t107303 + 0.12337005501361698274e-1_f64 * t96989 + 3.0_f64 * t6378 * t7747 - 3.0_f64 * t1336 * t26458 * t6415 + 0.78134368175290755733e-1_f64 * t90900 - 3.0_f64 * t5234 * t28174;
    t107314
}
