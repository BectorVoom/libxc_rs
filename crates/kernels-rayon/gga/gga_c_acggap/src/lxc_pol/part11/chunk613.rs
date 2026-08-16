//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 613/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk613(t1465: f64, t4680: f64, t1579: f64, t407: f64, t1181: f64, t1470: f64, t3382: f64, t1180: f64, t3460: f64, t3477: f64, t3479: f64, t3494: f64, t3504: f64, t3532: f64, t3551: f64, t3552: f64, t3556: f64, t3562: f64, t3563: f64, t418: f64, t4667: f64, t4670: f64, t4673: f64, t4675: f64, t4677: f64, t4679: f64) -> (f64, f64, f64) {
    let t4681 = t4680 * t1465;
    let t4684 = t1579 * t407;
    let t4685 = t1181 * t4684;
    let t4689 = 0.85748036236139473944e-3_f64 * t3382 * t1470;
    let t4690 = -0.42874018118069736972e-3_f64 * t3460 - 0.42874018118069736972e-3_f64 * t3477 - 0.21437009059034868486e-3_f64 * t3479 - 0.42874018118069736972e-2_f64 * t3494 - 7.0_f64 / 72.0_f64 * t3504 + 0.42874018118069736972e-3_f64 * t3532 - t3551 + 0.42874018118069736972e-3_f64 * t3552 + t3556 - t3562 - 0.20007875121765877254e-2_f64 * t3563 + 0.12862205435420921092e-2_f64 * t418 * t4667 + 0.40015750243531754508e-2_f64 * t4670 + t4673 - t4675 + t4677 + t4679 - 0.85748036236139473944e-3_f64 * t1180 * t4681 - 0.85748036236139473944e-3_f64 * t1180 * t4685 + t4689;
    (t4681, t4685, t4690)
}
