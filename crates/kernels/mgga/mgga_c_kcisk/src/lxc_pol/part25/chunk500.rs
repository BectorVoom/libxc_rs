//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 500/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk500<F: Float>(t1648: F, t682: F, t1824: F, t4629: F, t298: F, t446: F, t569: F) -> (F, F, F) {
    let t4630 = t682 * t1648;
    let t4631 = t4630 * t1824;
    let t4632 = t4629 * t4631;
    let t4636 = t298 * t446 * t569;
    (t4631, t4632, t4636)
}
