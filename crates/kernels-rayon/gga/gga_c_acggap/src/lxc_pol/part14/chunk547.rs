//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 547/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk547(t40: f64, t4068: f64, t119: f64, t1603: f64, t1308: f64, t872: f64, t1620: f64, t857: f64, t1605: f64, t310: f64, t1659: f64, t315: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4069 = t40 * t4068;
    let t4103 = t119 * t1603;
    let t4107 = 0.13170898365871023197e1_f64 * t1308 * t872;
    let t4113 = 0.26341796731742046394e1_f64 * t857 * t1620;
    let t4123 = 0.13170898365871023197e1_f64 * t310 * t1605;
    let t4130 = 0.13170898365871023197e1_f64 * t857 * t1659;
    let t4131 = t315 * t1603;
    (t4069, t4103, t4107, t4113, t4123, t4130, t4131)
}
