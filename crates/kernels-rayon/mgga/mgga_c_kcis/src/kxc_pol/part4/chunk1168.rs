//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1168/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1168(t14374: f64, t388: f64, t387: f64, t1187: f64, t1812: f64, t3354: f64, t10752: f64, t3226: f64, t5177: f64, t14578: f64, t3338: f64, t3337: f64) -> (f64, f64, f64, f64) {
    let t14799 = t388 * t14374;
    let t14800 = t387 * t14799;
    let t14801 = t1187 * t14800;
    let t14803 = t3354 * t1812;
    let t14804 = t1187 * t14803;
    let t14806 = t3226 * t10752;
    let t14807 = t14806 * t5177;
    let t14809 = t3338 * t14578;
    let t14810 = t3337 * t14809;
    (t14801, t14804, t14807, t14810)
}
