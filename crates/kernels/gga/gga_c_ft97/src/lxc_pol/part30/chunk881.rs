//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 881/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk881<F: Float>(t150133: F, t7511: F, t7512: F, t7515: F, t1434: F, t35336: F, t681: F, t1882: F, t35534: F, t33319: F, t3837: F, t42500: F, t6118: F, t141357: F, t2354: F, t446: F, t992: F) -> (F, F, F, F, F) {
    let t150136 = t7511 * t7512 * t7515 * t150133;
    let t150139 = t1434 * t681 * t35336;
    let t150140 = t1882 * t35534;
    let t150144 = t6118 * t42500 * t33319 * t3837;
    let t150148 = t446 * t2354 * t141357 * t992;
    (t150136, t150139, t150140, t150144, t150148)
}
