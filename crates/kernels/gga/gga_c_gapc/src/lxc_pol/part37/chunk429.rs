//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 429/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk429<F: Float>(t2268: F, t876: F, t2261: F, t770: F, t640: F, t769: F, t791: F, t4: F, t891: F, t2416: F, t768: F, t825: F, t126: F, t824: F, t190: F, t291: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2423 = t2268 * t876;
    let t2426 = t2261 * t770;
    let t2429 = t769 * t640;
    let t2430 = t791 * t2429;
    let t2431 = t891 * t4;
    let t2432 = t2416 * t2431;
    let t2435 = t768 * t825;
    let t2436 = t2435 * t126;
    let t2437 = t824 * t2436;
    let t2438 = t190 * t291;
    (t2423, t2426, t2430, t2431, t2432, t2435, t2436, t2437, t2438)
}
