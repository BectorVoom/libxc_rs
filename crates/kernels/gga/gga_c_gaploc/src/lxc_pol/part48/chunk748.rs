//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 748/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk748<F: Float>(t21502: F, t44707: F, t1841: F, t21501: F, t3614: F, t935: F) -> (F, F, F) {
    let t44708 = t21502 * t44707;
    let t44711 = 0.51270174867614828557e-2 * t1841 * t21501 * t44708;
    let t44712 = t3614 * t935;
    (t44708, t44711, t44712)
}
