//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1329/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1329<F: Float>(t116131: F, t116165: F, t116199: F, t116242: F, t116279: F, t116323: F, t116359: F, t116400: F, t116443: F, t116485: F, t116527: F, t116569: F, t116607: F, t116641: F, t116679: F, t116718: F, t116753: F, t116784: F, t116819: F, t116850: F, t116878: F, t116909: F, t116952: F, t116988: F, t117018: F, t117056: F, t117089: F, t117124: F, t117155: F, t117187: F, t117217: F, t117252: F, t752: F) -> (F,) {
    let t117258 = (t116131 + t116199 + t116165 + t116242 + t117187 + t116641 + t116443 + t117056 + t117155 + t116607 + t116359 + t116909 + t116784 + t116400 + t116952 + t116679 + t116753 + t117018 + t117124 + t116569 + t116323 + t116819 + t116527 + t116279 + t116988 + t117089 + t116485 + t117217 + t116850 + t116878 + t117252 + t116718) * t752;
    (t117258,)
}
