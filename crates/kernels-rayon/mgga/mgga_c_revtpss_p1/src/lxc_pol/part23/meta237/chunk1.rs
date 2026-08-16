//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1390/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1390(t1388: f64, t1410: f64, t3944: f64, t3950: f64, t3956: f64, t3967: f64, t5606: f64, t5625: f64, t5666: f64, t5681: f64, t6846: f64, t6850: f64, t6856: f64, t6887: f64) -> f64 {
    let t6888 = 7.0_f64 / 72.0_f64 * t5681 + 0.20007875121765877254e-2_f64 * t5625 - 0.21437009059034868486e-3_f64 * t1388 * t6846 + t3944 * t6850 / 16.0_f64 + t3950 + 0.80031500487063509015e-2_f64 * t5606 - 0.25410001404642664112e-4_f64 * t5666 - 0.85748036236139473944e-3_f64 * t1410 * t6856 + t3956 + t3967 + t6887;
    t6888
}
