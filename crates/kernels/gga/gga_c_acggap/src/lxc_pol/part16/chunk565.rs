//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 565/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk565<F: Float>(t530: F, t5616: F, t1181: F, t1759: F, t301: F, t1165: F, t1552: F, t1173: F, t1180: F, t335: F, t367: F, t418: F, t4340: F, t4350: F, t4361: F, t5561: F, t5570: F, t5574: F, t5577: F, t5579: F, t5581: F, t5583: F, t5586: F, t5590: F, t5594: F, t5598: F, t5601: F, t5603: F, t5608: F, t5612: F) -> (F, F, F) {
    let t5617 = t530 * t5616;
    let t5618 = t1181 * t5617;
    let t5621 = t1759 * t301;
    let t5623 = t1165 * t1552 * t5621;
    let t5626 = 0.85748036236139473944e-2 * t418 * t5561 + 0.40015750243531754508e-2 * t4340 + 0.17149607247227894789e-2 * t4350 - 0.34299214494455789578e-2 * t4361 - 0.12862205435420921092e-2 * t5570 - 0.51448821741683684368e-2 * t418 * t5574 - 0.20007875121765877254e-2 * t5577 + 0.16006300097412701803e-1 * t5579 - 0.16006300097412701803e-1 * t5581 + 0.80031500487063509015e-2 * t5583 - t367 * t5586 / 96.0 - t335 * t5590 / 24.0 - t335 * t5594 / 24.0 - t335 * t5598 / 48.0 - 0.20007875121765877254e-2 * t5601 + 0.42874018118069736972e-3 * t5603 - 0.34299214494455789578e-2 * t1173 * t5608 + 0.17149607247227894789e-2 * t1180 * t5612 + 0.34299214494455789578e-2 * t1173 * t5618 - 0.34299214494455789578e-2 * t1173 * t5623;
    (t5618, t5623, t5626)
}
