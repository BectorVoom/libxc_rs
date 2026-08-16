//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3863/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3863(t13847: f64, t1399: f64, t73856: f64, t9816: f64, t22298: f64, t48100: f64, t22129: f64, t2713: f64, t3964: f64, t22046: f64, t22079: f64, t3829: f64, t3934: f64, t4057: f64, t48548: f64, t48553: f64, t48557: f64, t48563: f64, t48565: f64, t5671: f64, t5673: f64, t6883: f64, t73847: f64, t800: f64, t9748: f64, t9840: f64) -> f64 {
    let t74249 = t9816 * t13847 * t73856 * t1399;
    let t74257 = t9816 * t48100 * t22298;
    let t74264 = t3964 * t2713 * t22129;
    let t74266 = 0.12862205435420921092e-2_f64 * t5671 * t5673 * t22046 * t9840 - 0.42874018118069736972e-3_f64 * t3934 * t5673 * t73847 * t1399 - 0.21437009059034868486e-3_f64 * t3934 * t5673 * t22079 * t4057 - 0.25410001404642664112e-4_f64 * t74249 + 0.2032800112371413129e-3_f64 * t48548 - t9748 * t800 * t6883 * t3829 / 4.0_f64 + 0.2032800112371413129e-3_f64 * t74257 - 0.11433071498151929859e-3_f64 * t48553 - 0.57165357490759649296e-4_f64 * t48557 - 0.40164115440237189888e-6_f64 * t48563 - 0.40015750243531754508e-1_f64 * t48565 + 0.90357964994909313586e-4_f64 * t74264;
    t74266
}
