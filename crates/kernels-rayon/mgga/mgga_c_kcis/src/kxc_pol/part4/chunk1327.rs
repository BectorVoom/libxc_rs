//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1327/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1327(t17040: f64, t3815: f64, t1445: f64, t5789: f64, t532: f64, t5793: f64, t1951: f64, t2645: f64, t5796: f64, t833: f64, t3841: f64, t5792: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17041 = t17040 * t3815;
    let t17045 = 0.93706135855523581992e-2_f64 * t1445 * t5789;
    let t17047 = 0.93706135855523581992e-2_f64 * t532 * t5793;
    let t17048 = t1951 * t2645;
    let t17051 = t5796 * t833;
    let t17054 = t5792 * t3841;
    (t17041, t17045, t17047, t17048, t17051, t17054)
}
