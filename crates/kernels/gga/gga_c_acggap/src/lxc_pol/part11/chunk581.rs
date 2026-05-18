//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 581/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk581<F: Float>(t159: F, t1603: F, t322: F, t381: F, t1639: F, t377: F, t550: F, t980: F, t1636: F, t553: F, t848: F, t394: F) -> (F, F, F, F, F, F) {
    let t4225 = t159 * t1603;
    let t4226 = t4225 * t322;
    let t4228 = F::new(0.13170898365871023197e1) * t381 * t4226;
    let t4230 = F::new(0.13170898365871023197e1) * t377 * t1639;
    let t4231 = t980 * t550;
    let t4234 = F::new(0.13170898365871023197e1) * t377 * t1636;
    let t4235 = t848 * t553;
    let t4237 = t394 * t1603;
    (t4228, t4230, t4231, t4234, t4235, t4237)
}
