//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 615/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk615(t4663: f64, t8504: f64, t1646: f64, t8522: f64, t4676: f64, t6756: f64, t8512: f64, t8516: f64, t8520: f64, t1815: f64, t2372: f64, t4664: f64, t574: f64, t6774: f64) -> (f64, f64, f64, f64) {
    let t8525 = t4663 * t8504;
    let t8527 = t1646 * t8522;
    let t8533 = -0.991e-2_f64 * t8525 + 0.1982e-1_f64 * t8527 + t4676 + 0.27516666666666666666e-2_f64 * t6756 - 0.27516666666666666667e-2_f64 * t8512 + 0.8255e-2_f64 * t8516 - 0.41275e-2_f64 * t8520;
    let t8536 = -t4664 * t8504 / 8.0_f64 + t6774 * t2372 / 2.0_f64 + t1815 * t8522 / 4.0_f64 + t574 * t8533 / 2.0_f64;
    (t8525, t8527, t8533, t8536)
}
