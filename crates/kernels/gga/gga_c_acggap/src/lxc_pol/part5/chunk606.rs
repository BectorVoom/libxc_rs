//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 606/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk606<F: Float>(t1659: F, t857: F, t1603: F, t315: F, t323: F, t310: F, t545: F) -> (F, F, F, F) {
    let t4130 = 0.13170898365871023197e1 * t857 * t1659;
    let t4131 = t315 * t1603;
    let t4133 = 0.13170898365871023197e1 * t4131 * t323;
    let t4137 = t310 * t545;
    (t4130, t4131, t4133, t4137)
}
