//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 552/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk552<F: Float>(t1629: F, t4210: F, t1160: F, t1652: F, t377: F, t159: F, t1603: F, t322: F, t381: F, t1639: F, t550: F, t980: F) -> (F, F, F, F, F) {
    let t4211 = t1629 * t4210;
    let t4213 = F::new(0.13170898365871023197e1) * t1160 * t4211;
    let t4215 = F::new(0.13170898365871023197e1) * t377 * t1652;
    let t4225 = t159 * t1603;
    let t4226 = t4225 * t322;
    let t4228 = F::new(0.13170898365871023197e1) * t381 * t4226;
    let t4230 = F::new(0.13170898365871023197e1) * t377 * t1639;
    let t4231 = t980 * t550;
    (t4213, t4215, t4228, t4230, t4231)
}
