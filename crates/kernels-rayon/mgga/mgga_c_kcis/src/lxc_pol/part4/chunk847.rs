//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 847/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk847(t1336: f64, t5541: f64, t1907: f64, t3856: f64, t1334: f64, t3861: f64, t1897: f64, t3873: f64, t1319: f64, t1324: f64, t5481: f64, t1903: f64, t659: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5543 = 1.0_f64 * t5541 * t1336;
    let t5545 = 1.0_f64 * t3856 * t1907;
    let t5546 = t1907 * t1334;
    let t5548 = 2.0_f64 * t3861 * t5546;
    let t5556 = t3873 * t1897;
    let t5557 = t5556 * t1319;
    let t5559 = t1324 * t5481;
    let t5562 = t659 * t1903;
    (t5543, t5545, t5546, t5548, t5556, t5557, t5559, t5562)
}
