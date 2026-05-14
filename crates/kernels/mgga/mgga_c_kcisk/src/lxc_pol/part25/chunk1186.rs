//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1186/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1186<F: Float>(t2029: F, t5507: F, t24991: F, t7261: F, t2028: F, t2642: F, t33197: F, t10009: F, t33208: F, t33297: F, t34390: F, t34395: F, t34400: F, t34406: F, t34412: F, t34416: F, t34419: F, t9740: F, t9743: F) -> (F, F, F, F, F, F) {
    let t34422 = t5507 * t2029;
    let t34423 = t34422 * t24991;
    let t34424 = t7261 * t34423;
    let t34427 = t2642 * t2028;
    let t34428 = t33197 * t34427;
    let t34429 = t7261 * t34428;
    let t34432 = -0.5787037037037037037e-3 * t34390 - 0.17361111111111111111e-2 * t33208 * t10009 + 0.34722222222222222222e-2 * t9740 * t34395 - 0.52083333333333333333e-2 * t9740 * t34400 - 0.10416666666666666667e-1 * t9740 * t34406 - 0.17361111111111111111e-2 * t33297 * t10009 + 0.46296296296296296297e-2 * t34412 * t9743 - 0.17361111111111111111e-2 * t34416 * t9743 - 0.116403125e-2 * t34419 * t34406 - 0.10416666666666666667e-1 * t9740 * t34424 - 0.52083333333333333333e-2 * t9740 * t34429;
    (t34422, t34423, t34424, t34428, t34429, t34432)
}
