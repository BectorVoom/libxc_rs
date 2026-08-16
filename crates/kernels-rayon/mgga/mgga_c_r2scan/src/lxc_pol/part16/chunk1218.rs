//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1218/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1218(t10760: f64, t30637: f64, t6085: f64, t30643: f64, t11678: f64, t7601: f64, t30140: f64, t30856: f64, t6093: f64, t12495: f64, t19872: f64, t29775: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43528 = t6085 * t10760 * t30637;
    let t43531 = t6085 * t10760 * t30643;
    let t43533 = t7601 * t11678;
    let t43536 = t6085 * t10760 * t30140;
    let t43539 = t6093 * t10760 * t30856;
    let t43541 = t19872 * t12495;
    let t43544 = t6093 * t10760 * t29775;
    (t43528, t43531, t43533, t43536, t43539, t43541, t43544)
}
