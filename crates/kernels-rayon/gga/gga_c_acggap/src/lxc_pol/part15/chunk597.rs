//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 597/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk597(t530: f64, t5616: f64, t1181: f64, t1759: f64, t301: f64, t1165: f64, t1552: f64, t1173: f64, t1180: f64, t335: f64, t367: f64, t418: f64, t4340: f64, t4350: f64, t4361: f64, t5561: f64, t5570: f64, t5574: f64, t5577: f64, t5579: f64, t5581: f64, t5583: f64, t5586: f64, t5590: f64, t5594: f64, t5598: f64, t5601: f64, t5603: f64, t5608: f64, t5612: f64) -> (f64, f64, f64) {
    let t5617 = t530 * t5616;
    let t5618 = t1181 * t5617;
    let t5621 = t1759 * t301;
    let t5623 = t1165 * t1552 * t5621;
    let t5626 = 0.85748036236139473944e-2_f64 * t418 * t5561 + 0.40015750243531754508e-2_f64 * t4340 + 0.17149607247227894789e-2_f64 * t4350 - 0.34299214494455789578e-2_f64 * t4361 - 0.12862205435420921092e-2_f64 * t5570 - 0.51448821741683684368e-2_f64 * t418 * t5574 - 0.20007875121765877254e-2_f64 * t5577 + 0.16006300097412701803e-1_f64 * t5579 - 0.16006300097412701803e-1_f64 * t5581 + 0.80031500487063509015e-2_f64 * t5583 - t367 * t5586 / 96.0_f64 - t335 * t5590 / 24.0_f64 - t335 * t5594 / 24.0_f64 - t335 * t5598 / 48.0_f64 - 0.20007875121765877254e-2_f64 * t5601 + 0.42874018118069736972e-3_f64 * t5603 - 0.34299214494455789578e-2_f64 * t1173 * t5608 + 0.17149607247227894789e-2_f64 * t1180 * t5612 + 0.34299214494455789578e-2_f64 * t1173 * t5618 - 0.34299214494455789578e-2_f64 * t1173 * t5623;
    (t5618, t5623, t5626)
}
