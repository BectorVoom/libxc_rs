//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1066/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1066(t1583: f64, t546: f64, t2078: f64, t3320: f64, t783: f64, t787: f64, t1266: f64, t512: f64, t57: f64, t1607: f64, t10856: f64, t6271: f64) -> (f64, f64, f64, f64, f64) {
    let t37685 = t546 * t1583;
    let t37696 = t783 * t2078 * t787 * t3320;
    let t37699 = t512 * t1266 * t57;
    let t37700 = t37699 * t1607;
    let t37702 = t10856 * t6271;
    (t37685, t37696, t37699, t37700, t37702)
}
