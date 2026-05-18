//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1122/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1122<F: Float>(t11917: F, t29481: F, t3402: F, t2387: F, t3750: F, t3752: F, t33530: F, t3430: F, t6188: F, t11853: F, t291: F, t8685: F) -> (F, F, F, F) {
    let t33875 = t3402 * t11917 * t29481;
    let t33878 = t2387 * t3750 * t3752;
    let t33881 = t3430 * t33530 * t6188;
    let t33884 = t8685 * t291 * t11853;
    (t33875, t33878, t33881, t33884)
}
