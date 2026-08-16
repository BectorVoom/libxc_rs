//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 572/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk572(t119: f64, t1603: f64, t1308: f64, t872: f64, t1221: f64, t3875: f64, t556: f64, t1620: f64, t857: f64, t1658: f64, t463: f64, t1220: f64) -> (f64, f64, f64, f64, f64) {
    let t4103 = t119 * t1603;
    let t4107 = 0.13170898365871023197e1_f64 * t1308 * t872;
    let t4109 = t3875 * t556 * t1221;
    let t4113 = 0.26341796731742046394e1_f64 * t857 * t1620;
    let t4118 = t1658 * t463;
    let t4119 = t1220 * t4118;
    (t4103, t4107, t4109, t4113, t4119)
}
