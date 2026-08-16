//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1923/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1923(t27883: f64, t786: f64, t7286: f64, t1903: f64, t7274: f64, t7296: f64, t25902: f64, t25905: f64, t25914: f64, t25919: f64, t25921: f64, t25941: f64, t25948: f64, t25951: f64, t27885: f64, t27889: f64, t27891: f64, t27896: f64, t7295: f64, t7921: f64) -> (f64, f64, f64, f64) {
    let t27899 = t786 * t27883;
    let t27900 = t27899 * t7286;
    let t27902 = t7274 * t1903;
    let t27903 = t7296 * t27902;
    let t27907 = 0.12851425765524037203e-1_f64 * t25902 - 0.72280234901709995518e-2_f64 * t25905 - 0.54878743191129263322e-2_f64 * t25914 - t25919 - 0.12851425765524037203e-1_f64 * t27885 + 0.72280234901709995518e-2_f64 * t27889 - 0.12851425765524037203e-1_f64 * t27891 + 0.8673628188205199462e0_f64 * t25921 * t7921 + 0.8673628188205199462e0_f64 * t7295 * t27896 + 0.72280234901709995518e-2_f64 * t27900 + 0.8673628188205199462e0_f64 * t7295 * t27903 - t25941 + t25948 - 0.12851425765524037203e-1_f64 * t25951;
    (t27899, t27902, t27903, t27907)
}
