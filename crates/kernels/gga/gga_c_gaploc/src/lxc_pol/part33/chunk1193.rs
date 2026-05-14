//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1193/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1193<F: Float>(t2366: F, t38276: F, t12000: F, t158: F, t1063: F, t12008: F, t123: F, t1349: F, t1358: F, t29850: F, t29852: F, t31488: F, t31490: F, t31493: F, t31496: F, t31498: F, t31522: F, t3808: F, t38267: F, t38272: F, t38277: F, t4323: F, t488: F, t535: F, t6507: F) -> (F, F, F) {
    let t38281 = t2366 * t38276;
    let t38285 = t158 * t12000;
    let t38292 = -0.56910013271352299198e-1 * t1063 * t535 * t38267 - 0.63233348079280332442e-2 * t1349 * t4323 * t38272 - 0.12646669615856066488e-1 * t1358 * t6507 * t38277 + 0.18970004423784099733e-1 * t1358 * t4323 * t38281 - 0.63233348079280332442e-2 * t1358 * t38285 * t123 * t488 + 0.63233348079280332442e-2 * t3808 * t12008 - t31488 + t31490 + t31493 + t31496 + t29850 - t29852 + t31498 + t31522;
    (t38281, t38285, t38292)
}
