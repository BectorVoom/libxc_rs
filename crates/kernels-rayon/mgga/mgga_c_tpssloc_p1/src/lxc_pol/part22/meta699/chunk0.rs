//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2282/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2282(t1222: f64, t18574: f64, t11789: f64, t1227: f64, t248: f64, t5975: f64, t18321: f64, t3548: f64, t15437: f64, t15502: f64, t15506: f64, t4965: f64, t5023: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65681 = t18574 * t1222;
    let t65689 = t1227 * t248 * t11789 * t5975;
    let t65691 = t18321 * t3548;
    let t65703 = t15437 * t15502;
    let t65706 = t15437 * t15506;
    let t65709 = t4965 * t5023;
    (t65681, t65689, t65691, t65703, t65706, t65709)
}
