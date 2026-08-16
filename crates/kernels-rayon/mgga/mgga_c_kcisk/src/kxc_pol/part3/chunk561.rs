//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 561/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk561(t4663: f64, t677: f64, t1646: f64, t1821: f64, t4624: f64, t4652: f64, t4636: f64, t4638: f64, t4642: f64, t4646: f64, t4650: f64, t1648: f64, t1815: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4664 = t4663 * t677;
    let t4667 = t1646 * t1821;
    let t4672 = t4663 * t4624;
    let t4674 = t1646 * t4652;
    let t4676 = 0.55033333333333333333e-2_f64 * t4636;
    let t4681 = -0.991e-2_f64 * t4672 + 0.1982e-1_f64 * t4674 + t4676 + 0.27516666666666666666e-2_f64 * t4638 - 0.27516666666666666667e-2_f64 * t4642 + 0.8255e-2_f64 * t4646 - 0.41275e-2_f64 * t4650;
    let t4684 = -t4664 * t4624 / 8.0_f64 + t4667 * t1648 / 2.0_f64 + t1815 * t4652 / 4.0_f64 + t574 * t4681 / 2.0_f64;
    (t4664, t4667, t4672, t4674, t4681, t4684)
}
