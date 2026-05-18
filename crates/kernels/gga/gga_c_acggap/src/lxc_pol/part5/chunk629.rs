//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 629/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk629<F: Float>(t1533: F, t4146: F, t1539: F, t1160: F, t1629: F, t930: F, t3084: F, t157: F, t879: F) -> (F, F, F, F, F, F) {
    let t4147 = t4146 * t1533;
    let t4150 = t4146 * t1539;
    let t4152 = F::new(0.13170898365871023197e1) * t1160 * t4150;
    let t4153 = t1629 * t930;
    let t4159 = t1629 * t3084;
    let t4162 = t157 * t879;
    (t4147, t4150, t4152, t4153, t4159, t4162)
}
