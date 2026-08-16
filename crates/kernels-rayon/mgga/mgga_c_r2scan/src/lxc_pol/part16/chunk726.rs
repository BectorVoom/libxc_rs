//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 726/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk726(t5836: f64, t61: f64, t1748: f64, t1751: f64, t1398: f64, t745: f64, t735: f64, t1668: f64, t591: f64, t615: f64, t1757: f64, t1679: f64, t584: f64) -> (f64, f64, f64, f64, f64) {
    let t5925 = 0.3903689268108626343e0_f64 * t61 * t5836;
    let t5936 = t1751 * t1748;
    let t5938 = t1398 * t745;
    let t5940 = 0.21687162600603479684e-1_f64 * t735 * t5938;
    let t5942 = t1668 * t591;
    let t5943 = t615 * t5942;
    let t5945 = 0.67745118933333333331e-2_f64 * t1757 * t5943;
    let t5946 = t584 * t1679;
    (t5925, t5936, t5940, t5945, t5946)
}
