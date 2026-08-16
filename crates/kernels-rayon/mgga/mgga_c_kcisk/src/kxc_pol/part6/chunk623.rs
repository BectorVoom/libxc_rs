//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 623/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk623(t1856: f64, t8514: f64, t1842: f64, t8518: f64, t1659: f64, t8510: f64, t1835: f64, t165: f64, t173: f64, t5122: f64, t5125: f64, t5128: f64, t5129: f64, t5135: f64, t5168: f64, t7715: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8620 = t1856 * t8514;
    let t8623 = t1842 * t8518;
    let t8626 = t1659 * t8510;
    let t8629 = t1856 * t8518;
    let t8632 = t1835 * t8510;
    let t8637 = t5122 - t5125 - t5128 - 0.10082625e-4_f64 * t173 * t8620 + 0.7925e-3_f64 * t165 * t8623 - 0.52833333333333333333e-3_f64 * t165 * t8626 + 0.50413125e-5_f64 * t173 * t8629 - 0.672175e-5_f64 * t173 * t8632 - t5129 + t5135 + 0.15538616723388920628e-3_f64 * t5168 * t7715;
    (t8620, t8623, t8626, t8629, t8632, t8637)
}
