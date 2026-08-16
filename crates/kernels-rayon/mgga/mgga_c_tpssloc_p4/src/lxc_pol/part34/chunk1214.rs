//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1214/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1214(t102587: f64, t107281: f64, t107303: f64, t107320: f64, t107326: f64, t1825: f64, t20473: f64, t27074: f64, t5334: f64, t5344: f64, t84480: f64, t84481: f64, t90807: f64, t90837: f64, t90868: f64, t90900: f64, t96937: f64, t96945: f64, t96989: f64, t97026: f64, t97043: f64, t97049: f64) -> f64 {
    let t107908 = -0.23029076935875170111e0_f64 * t96937 - 0.76763589786250567036e0_f64 * t90807 + 0.11514538467937585055e0_f64 * t96945 - 0.49348022005446793095e-1_f64 * t107281 - 0.31253747270116302294e0_f64 * t90837 + 6.0_f64 * t5334 * t27074 * t20473 + 0.38381794893125283518e0_f64 * t90868 - 3.0_f64 * t5344 * t102587 * t1825 + 0.9869604401089358619e-1_f64 * t107303 + 0.24674011002723396548e-1_f64 * t96989 + 0.15626873635058151147e0_f64 * t90900 - t84480 - t84481 - 0.16449340668482264365e-1_f64 * t107320 + 0.49348022005446793095e-1_f64 * t97026 - 0.9869604401089358619e-1_f64 * t97043 - 0.49348022005446793095e-1_f64 * t97049 - 0.49348022005446793095e-1_f64 * t107326;
    t107908
}
