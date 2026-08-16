//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1526/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1526(t3075: f64, t5004: f64, t359: f64, t4930: f64, t999: f64, t1043: f64, t1089: f64, t4757: f64, t3291: f64, t4772: f64, t1678: f64, t3133: f64) -> (f64, f64, f64, f64, f64) {
    let t16446 = t5004 * t3075;
    let t16449 = t359 * t4930;
    let t16450 = t16449 * t999;
    let t16458 = t4757 * t1043 * t1089;
    let t16461 = t3291 * t4772;
    let t16465 = t1678 * t3133 * t1089;
    (t16446, t16450, t16458, t16461, t16465)
}
