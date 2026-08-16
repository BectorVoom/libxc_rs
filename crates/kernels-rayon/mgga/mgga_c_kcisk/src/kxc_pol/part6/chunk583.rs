//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 583/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk583(t457: f64, t7736: f64, t1383: f64, t7744: f64, t1186: f64, t1398: f64, t1375: f64, t7740: f64, t158: f64, t165: f64, t173: f64, t3819: f64, t3891: f64, t5831: f64, t5833: f64, t5836: f64, t7706: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8130 = t457 * t7736;
    let t8133 = t1383 * t7744;
    let t8136 = t1186 * t7736;
    let t8139 = t1398 * t7744;
    let t8142 = t1375 * t7736;
    let t8145 = t1375 * t7740;
    let t8148 = t1383 * t7740;
    let t8158 = 0.1171e-2_f64 * t158 * t8130 + 0.7925e-3_f64 * t165 * t8133 - 0.52833333333333333333e-3_f64 * t165 * t8136 + 0.50413125e-5_f64 * t173 * t8139 - 0.672175e-5_f64 * t173 * t8142 + 0.7026e-2_f64 * t158 * t8145 - 0.1585e-2_f64 * t165 * t8148 - 0.23911438650126355246e-1_f64 * t3819 * t7706 + 0.15538616723388920628e-3_f64 * t3891 * t7706 + 0.9368e-2_f64 * t5831 - 0.26416666666666666666e-2_f64 * t5833 - 0.23526125e-4_f64 * t5836;
    (t8130, t8133, t8136, t8139, t8142, t8145, t8148, t8158)
}
