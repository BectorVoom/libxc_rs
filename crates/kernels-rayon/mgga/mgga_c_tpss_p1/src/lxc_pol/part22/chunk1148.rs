//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1148/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1148(t10019: f64, t10025: f64, t10028: f64, t10236: f64, t1196: f64, t1206: f64, t12754: f64, t12755: f64, t12756: f64, t12757: f64, t12759: f64, t12760: f64, t12764: f64, t12769: f64, t12770: f64, t12775: f64, t12779: f64, t12780: f64, t12810: f64, t1625: f64, t198: f64, t3183: f64, t3234: f64, t4528: f64, t4532: f64, t9972: f64, t9980: f64) -> f64 {
    let t12814 = -3.0_f64 * t10236 * t1625 * t3183 + 3.0_f64 * t1196 * t12810 * t198 + 12.0_f64 * t1206 * t12760 * t4532 + 3.0_f64 * t3183 * t3234 * t4528 + 6.0_f64 * t12764 * t4532 - t10019 + t10025 - t10028 - t12754 - t12755 - t12756 + t12757 + t12759 - t12769 - t12770 + t12775 - t12779 + t12780 - t9972 - t9980;
    t12814
}
