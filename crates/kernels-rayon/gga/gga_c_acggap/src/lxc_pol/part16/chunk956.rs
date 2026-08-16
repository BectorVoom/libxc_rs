//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 956/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk956(t154: f64, t506: f64, t7322: f64, t7326: f64, t7315: f64, t8589: f64, t30268: f64, t8775: f64, t30105: f64, t8952: f64, t7839: f64, t8739: f64) -> (f64, f64, f64, f64, f64) {
    let t33960 = t7322 * t154 * t506 * t7326;
    let t33962 = t7315 * t8589;
    let t33963 = 11.0_f64 / 192.0_f64 * t33962;
    let t33982 = t30268 * t8775;
    let t33983 = 0.64311027177104605458e-2_f64 * t33982;
    let t33984 = t30105 * t8952;
    let t33986 = t7839 * t8739;
    (t33960, t33963, t33983, t33984, t33986)
}
