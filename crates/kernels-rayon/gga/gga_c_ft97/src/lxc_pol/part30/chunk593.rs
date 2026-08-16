//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 593/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk593(t14842: f64, t420: f64, t27671: f64, t24378: f64, t6034: f64, t6804: f64, t17864: f64, t232: f64, t27552: f64, t1417: f64, t1701: f64, t17987: f64, t2035: f64, t24265: f64, t24332: f64, t24361: f64, t27500: f64, t27616: f64, t27621: f64, t27625: f64, t27629: f64, t27634: f64, t27638: f64, t27642: f64, t27647: f64, t27651: f64, t27653: f64, t27658: f64, t27662: f64, t27665: f64, t27670: f64, t6035: f64, t6037: f64) -> (f64, f64, f64, f64) {
    let t27672 = t420 * t14842;
    let t27673 = t27671 * t27672;
    let t27677 = t6034 * t24378 * t6804;
    let t27679 = t232 * t17864;
    let t27682 = t232 * t27552;
    let t27685 = 0.52801466802079540469e-5_f64 * t27616 * t27621 + 0.12768721675925925926e-1_f64 * t24332 + 0.52700762016626893448e-4_f64 * t17987 * t2035 * t27625 + 0.22227677429409423704e-2_f64 * t1417 * t1701 * t27629 - 0.85124811172839506173e-2_f64 * t27500 * t27634 + 0.12768721675925925926e-1_f64 * t24361 * t6035 * t27638 - 0.59387071557258112888e-3_f64 * t6034 * t27642 * t6037 + 0.12768721675925925926e-1_f64 * t24361 * t6035 * t27647 - 0.12768721675925925926e-1_f64 * t27651 * t6035 * t27653 - 0.15137014751914110597e-3_f64 * t27658 * t27662 + 0.22270151833971792333e-3_f64 * t6034 * t6035 * t27665 - 0.51789017496114396277e-5_f64 * t27670 * t27673 + 0.7423383944657264111e-4_f64 * t27677 - 0.44540303667943584666e-3_f64 * t24265 * t27679 - 0.44540303667943584666e-3_f64 * t24265 * t27682;
    (t27672, t27679, t27682, t27685)
}
