//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 593/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk593<F: Float>(t14842: F, t420: F, t27671: F, t24378: F, t6034: F, t6804: F, t17864: F, t232: F, t27552: F, t1417: F, t1701: F, t17987: F, t2035: F, t24265: F, t24332: F, t24361: F, t27500: F, t27616: F, t27621: F, t27625: F, t27629: F, t27634: F, t27638: F, t27642: F, t27647: F, t27651: F, t27653: F, t27658: F, t27662: F, t27665: F, t27670: F, t6035: F, t6037: F) -> (F, F, F, F) {
    let t27672 = t420 * t14842;
    let t27673 = t27671 * t27672;
    let t27677 = t6034 * t24378 * t6804;
    let t27679 = t232 * t17864;
    let t27682 = t232 * t27552;
    let t27685 = F::cast_from(0.52801466802079540469e-5_f64) * t27616 * t27621 + F::cast_from(0.12768721675925925926e-1_f64) * t24332 + F::cast_from(0.52700762016626893448e-4_f64) * t17987 * t2035 * t27625 + F::cast_from(0.22227677429409423704e-2_f64) * t1417 * t1701 * t27629 - F::cast_from(0.85124811172839506173e-2_f64) * t27500 * t27634 + F::cast_from(0.12768721675925925926e-1_f64) * t24361 * t6035 * t27638 - F::cast_from(0.59387071557258112888e-3_f64) * t6034 * t27642 * t6037 + F::cast_from(0.12768721675925925926e-1_f64) * t24361 * t6035 * t27647 - F::cast_from(0.12768721675925925926e-1_f64) * t27651 * t6035 * t27653 - F::cast_from(0.15137014751914110597e-3_f64) * t27658 * t27662 + F::cast_from(0.22270151833971792333e-3_f64) * t6034 * t6035 * t27665 - F::cast_from(0.51789017496114396277e-5_f64) * t27670 * t27673 + F::cast_from(0.7423383944657264111e-4_f64) * t27677 - F::cast_from(0.44540303667943584666e-3_f64) * t24265 * t27679 - F::cast_from(0.44540303667943584666e-3_f64) * t24265 * t27682;
    (t27672, t27679, t27682, t27685)
}
