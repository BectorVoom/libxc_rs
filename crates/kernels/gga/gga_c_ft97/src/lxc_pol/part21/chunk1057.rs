//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1057/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1057<F: Float>(t3018: F, t3020: F, t5566: F, t7857: F, t47: F, t8: F, t420: F, t22632: F, t22761: F, t25787: F, t22541: F, t22572: F, t25699: F, t12486: F, t173: F, t25753: F, t25754: F) -> (F, F, F, F, F) {
    let t101247 = t7857 * t3018 * t3020 * t5566;
    let t101248 = t8 * t47;
    let t101249 = t101248 * t420;
    let t101295 = t22761 * t22632 * t25787;
    let t101360 = t22541 * t22572 * t25699;
    let t101387 = t25753 * t25754 * t173 * t12486;
    (t101247, t101249, t101295, t101360, t101387)
}
