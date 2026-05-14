//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1032/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1032<F: Float>(t2387: F, t3750: F, t3752: F, t33530: F, t3430: F, t6188: F, t11853: F, t291: F, t8685: F, t9644: F, t11933: F, t3273: F, t869: F, t11964: F, t26447: F, t29314: F, t311: F) -> (F, F, F, F, F, F) {
    let t33878 = t2387 * t3750 * t3752;
    let t33881 = t3430 * t33530 * t6188;
    let t33884 = t8685 * t291 * t11853;
    let t33885 = t9644 * t33884;
    let t33888 = t869 * t11933 * t3273;
    let t33893 = t311 * t11964 * t26447 * t29314;
    (t33878, t33881, t33884, t33885, t33888, t33893)
}
