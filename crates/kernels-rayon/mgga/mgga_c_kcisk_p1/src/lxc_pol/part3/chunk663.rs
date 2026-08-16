//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 663/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk663(t4798: f64, t4817: f64, t1869: f64, t4805: f64, t4811: f64, t1865: f64, t3805: f64, t167: f64, t3281: f64, t10449: f64, t8: f64, t1899: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10512 = t4817 * t4798;
    let t10513 = t1869 * t10512;
    let t10515 = t4811 * t4805;
    let t10517 = t3805 * t1865;
    let t10519 = 6.0_f64 * t167;
    let t10520 = 6.0_f64 * t3281;
    let t10522 = t10449 * t8 + t10519 - t10520;
    let t10523 = t1899 * t10522;
    (t10513, t10515, t10517, t10519, t10520, t10522, t10523)
}
