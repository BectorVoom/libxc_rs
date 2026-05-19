//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 547/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk547<F: Float>(t40: F, t4068: F, t119: F, t1603: F, t1308: F, t872: F, t1620: F, t857: F, t1605: F, t310: F, t1659: F, t315: F) -> (F, F, F, F, F, F, F) {
    let t4069 = t40 * t4068;
    let t4103 = t119 * t1603;
    let t4107 = F::cast_from(0.13170898365871023197e1_f64) * t1308 * t872;
    let t4113 = F::cast_from(0.26341796731742046394e1_f64) * t857 * t1620;
    let t4123 = F::cast_from(0.13170898365871023197e1_f64) * t310 * t1605;
    let t4130 = F::cast_from(0.13170898365871023197e1_f64) * t857 * t1659;
    let t4131 = t315 * t1603;
    (t4069, t4103, t4107, t4113, t4123, t4130, t4131)
}
