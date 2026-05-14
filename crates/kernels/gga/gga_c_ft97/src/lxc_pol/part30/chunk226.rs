//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 226/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk226<F: Float>(t209: F, t625: F, t228: F, t231: F, t173: F, t705: F, t701: F, t191: F, t668: F, t2347: F, t2360: F, t703: F, t754: F, t761: F) -> (F, F, F, F, F, F, F) {
    let t2432 = t209 * t625;
    let t2434 = t228 * t2432 * t231;
    let t2435 = 0.42562405586419753087e-2 * t2434;
    let t2436 = t173 * t705;
    let t2437 = t701 * t2436;
    let t2440 = 1.0 / t191 / t668;
    let t2441 = t2440 * t2347;
    let t2446 = t703 * t2360;
    let t2469 = t754 * t761;
    (t2434, t2435, t2437, t2440, t2441, t2446, t2469)
}
