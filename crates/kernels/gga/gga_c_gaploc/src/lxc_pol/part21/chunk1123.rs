//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1123/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1123<F: Float>(t123: F, t21888: F, t7297: F, t9647: F, t16880: F, t21504: F, t29439: F, t9752: F, t23292: F, t2558: F, t1222: F, t3130: F) -> (F, F, F, F, F) {
    let t29498 = F::new(0.7690526230142224284e-2) * t9647 * t21888 * t123 * t7297;
    let t29501 = F::new(0.3845263115071112142e-2) * t9647 * t16880 * t21504;
    let t29503 = F::new(0.1281754371690370714e-2) * t29439 * t9752;
    let t29631 = F::new(0.64087718584518535698e-3) * t9647 * t23292 * t2558;
    let t29850 = F::new(0.63233348079280332442e-2) * t1222 * t3130;
    (t29498, t29501, t29503, t29631, t29850)
}
