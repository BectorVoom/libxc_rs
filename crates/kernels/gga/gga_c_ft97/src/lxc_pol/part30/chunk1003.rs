//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1003/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1003<F: Float>(t1882: F, t35534: F, t33319: F, t3837: F, t42500: F, t6118: F, t141357: F, t2354: F, t446: F, t992: F, t150096: F, t150099: F, t150102: F, t150106: F, t150109: F, t150114: F, t150118: F, t150122: F, t150125: F, t150128: F, t150131: F, t150136: F, t150139: F) -> (F, F, F, F) {
    let t150140 = t1882 * t35534;
    let t150144 = t6118 * t42500 * t33319 * t3837;
    let t150148 = t446 * t2354 * t141357 * t992;
    let t150150 = -F::new(8.0) / F::new(3.0) * t150096 + F::new(4.0) / F::new(3.0) * t150099 - F::new(4.0) / F::new(3.0) * t150102 - t150106 - F::new(6.0) * t150109 - F::new(4.0) / F::new(3.0) * t150114 + F::new(2.0) / F::new(3.0) * t150118 - F::new(2.0) / F::new(3.0) * t150122 - F::new(2.0) / F::new(3.0) * t150125 - F::new(2.0) / F::new(3.0) * t150128 - F::new(8.0) / F::new(3.0) * t150131 - F::new(2.0) * t150136 + t150139 - t150140 / F::new(9.0) + F::new(12.0) * t150144 + t150148 / F::new(3.0);
    (t150140, t150144, t150148, t150150)
}
