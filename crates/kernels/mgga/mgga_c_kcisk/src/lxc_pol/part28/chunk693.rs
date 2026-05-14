//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 693/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk693<F: Float>(t1814: F, t8536: F, t4691: F, t6756: F, t8512: F, t8516: F, t8520: F, t587: F, t2382: F, t6802: F, t2381: F, t1664: F, t4704: F, t4716: F, t8504: F, t1653: F, t8522: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8537 = t1814 * t8536;
    let t8544 = t4691 + 0.11872222222222222222e-1 * t6756 - 0.11872222222222222222e-1 * t8512 + 0.35616666666666666666e-1 * t8516 - 0.17808333333333333333e-1 * t8520;
    let t8546 = 0.62182e-1 * t8544 * t587;
    let t8548 = 2.0 * t6802 * t2382;
    let t8549 = t2381 * t2381;
    let t8550 = t8549 * t1664;
    let t8552 = 2.0 * t4704 * t8550;
    let t8559 = t4716 * t8504;
    let t8561 = t1653 * t8522;
    (t8537, t8544, t8546, t8548, t8549, t8550, t8552, t8559, t8561)
}
