//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 689/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk689(t10487: f64, t786: f64, t1849: f64, t2020: f64, t10791: f64, t397: f64, t782: f64, t2019: f64, t657: f64, t163: f64, t4597: f64, t2040: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12198 = t786 * t10487;
    let t12234 = t2020 * t1849;
    let t12246 = t397 * t10791 * t786;
    let t12248 = 0.9994882620098509563e-2_f64 * t782 * t12246;
    let t12253 = t2019 * t2019;
    let t12254 = 1.0_f64 / t12253;
    let t12255 = t657 * t12254;
    let t12261 = t397 * t163;
    let t12271 = t2020 * t4597;
    let t12350 = t2040 * t2040;
    (t12198, t12234, t12248, t12255, t12261, t12271, t12350)
}
