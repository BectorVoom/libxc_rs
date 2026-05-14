//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1302/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1302<F: Float>(t1008: F, t2035: F, t23728: F, t3379: F, t5790: F, t3404: F, t1691: F, t34871: F, t420: F, t26604: F, t26762: F, t22632: F, t26607: F, t5813: F, t12512: F, t1701: F, t5546: F) -> (F, F, F, F, F, F, F) {
    let t105102 = t2035 * t23728 * t1008;
    let t105106 = t2035 * t5790 * t3379;
    let t105110 = t2035 * t5790 * t3404;
    let t105117 = t420 * t34871 * t1691;
    let t105124 = t26604 * t26762;
    let t105127 = t5813 * t22632 * t26607;
    let t105130 = t1701 * t5546 * t12512;
    (t105102, t105106, t105110, t105117, t105124, t105127, t105130)
}
