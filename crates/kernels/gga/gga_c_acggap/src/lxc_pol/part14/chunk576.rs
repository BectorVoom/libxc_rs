//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 576/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk576<F: Float>(t406: F, t5752: F, t1532: F, t1181: F, t1881: F, t997: F, t1173: F, t1531: F, t3209: F, t3215: F, t3218: F, t3229: F, t3231: F, t3233: F, t3238: F, t3240: F, t3403: F, t3462: F, t4459: F, t4462: F, t5712: F, t5717: F, t5722: F, t5728: F, t5733: F, t5737: F, t5743: F, t5749: F) -> (F, F) {
    let t5753 = t5752 * t406;
    let t5754 = t1532 * t5753;
    let t5755 = t1181 * t5754;
    let t5758 = t997 * t1881;
    let t5766 = 0.34299214494455789578e-2 * t1173 * t5712 - 0.34299214494455789578e-2 * t1173 * t5717 + 0.17149607247227894789e-2 * t1173 * t5722 + 0.85748036236139473944e-3 * t5728 - 0.17149607247227894789e-2 * t5733 - 0.85748036236139473945e-2 * t3403 * t5737 - 0.17149607247227894789e-2 * t1531 * t5743 - 0.34299214494455789578e-2 * t3462 * t5749 + 0.17149607247227894789e-2 * t1531 * t5755 - t3209 - 0.60023625365297631763e-2 * t5758 + 0.85748036236139473944e-3 * t4459 + t4462 - t3215 - t3218 - 0.85748036236139473944e-3 * t3229 + 0.42874018118069736972e-3 * t3231 - 0.42874018118069736972e-3 * t3233 - 0.40015750243531754508e-2 * t3238 + 0.40015750243531754508e-2 * t3240;
    (t5755, t5766)
}
