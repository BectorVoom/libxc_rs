//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1208/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1208<F: Float>(t10009: F, t33208: F, t33297: F, t34390: F, t34395: F, t34400: F, t34406: F, t34412: F, t34416: F, t34419: F, t34424: F, t34429: F, t9740: F, t9743: F, t9739: F, t9990: F) -> (F, F) {
    let t34432 = -0.5787037037037037037e-3 * t34390 - 0.17361111111111111111e-2 * t33208 * t10009 + 0.34722222222222222222e-2 * t9740 * t34395 - 0.52083333333333333333e-2 * t9740 * t34400 - 0.10416666666666666667e-1 * t9740 * t34406 - 0.17361111111111111111e-2 * t33297 * t10009 + 0.46296296296296296297e-2 * t34412 * t9743 - 0.17361111111111111111e-2 * t34416 * t9743 - 0.116403125e-2 * t34419 * t34406 - 0.10416666666666666667e-1 * t9740 * t34424 - 0.52083333333333333333e-2 * t9740 * t34429;
    let t34435 = t9990 * t9739;
    (t34432, t34435)
}
