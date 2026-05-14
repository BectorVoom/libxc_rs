//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1132/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1132<F: Float>(t2804: F, t33163: F, t33165: F, t33168: F, t33173: F, t33177: F, t33180: F, t33183: F, t33188: F, t33193: F, t33196: F, t33200: F, t33204: F, t33208: F, t9721: F, t9728: F, t9740: F, t9743: F, t9748: F) -> (F,) {
    let t33211 = 0.10416666666666666667e-1 * t9721 * t9748 + 0.13402777777777777778e-2 * t33163 - 0.34722222222222222222e-2 * t33165 - 0.11574074074074074074e-2 * t33168 + 0.10416666666666666667e-1 * t9721 * t9728 - 0.17361111111111111111e-2 * t9740 * t33173 - 0.116403125e-2 * t33177 * t33180 + 0.40208333333333333334e-2 * t33183 * t9728 + 0.52083333333333333333e-2 * t2804 * t33188 + 0.52083333333333333333e-2 * t2804 * t33193 - 0.40208333333333333334e-2 * t33196 * t33200 - 0.23148148148148148148e-2 * t9740 * t33204 - 0.34722222222222222222e-2 * t33208 * t9743;
    (t33211,)
}
