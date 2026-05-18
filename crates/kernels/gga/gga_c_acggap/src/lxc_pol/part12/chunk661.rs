//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 661/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk661<F: Float>(t3896: F, t557: F, t545: F, t851: F, t323: F, t1614: F, t868: F, t1308: F, t880: F, t449: F, t556: F, t879: F) -> (F, F, F, F, F) {
    let t5359 = F::new(0.13170898365871023197e1) * t3896 * t557;
    let t5360 = t851 * t545;
    let t5361 = t5360 * t323;
    let t5364 = F::new(0.13170898365871023197e1) * t868 * t1614;
    let t5365 = t1308 * t880;
    let t5368 = t449 * t556 * t879;
    (t5359, t5361, t5364, t5365, t5368)
}
