//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 736/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk736(t1784: f64, t584: f64, t591: f64, t1789: f64, t406: f64, t410: f64, t1748: f64, t1751: f64, t1398: f64, t745: f64, t735: f64, t1668: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5930 = t584 * t1784 * t591;
    let t5932 = t406 * t1789;
    let t5934 = t410 * t1789;
    let t5936 = t1751 * t1748;
    let t5938 = t1398 * t745;
    let t5940 = 0.21687162600603479684e-1_f64 * t735 * t5938;
    let t5942 = t1668 * t591;
    (t5930, t5932, t5934, t5936, t5940, t5942)
}
