//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1306/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1306<F: Float>(t1333: F, t34174: F, t116723: F, t116726: F, t116731: F, t116737: F, t116738: F, t116741: F, t116743: F, t116745: F, t2781: F, t2785: F, t32921: F, t32948: F, t33031: F, t33056: F, t34148: F, t64926: F) -> (F, F) {
    let t116747 = t1333 * t34174;
    let t116748 = 0.33163888888888888888e-2 * t116747;
    let t116753 = -0.73697530864197530861e-3 * t116723 - 0.69444444444444444446e-2 * t33031 * t116726 - 0.26805555555555555556e-2 * t33056 * t116726 - 0.55273148148148148147e-3 * t116731 - 0.10416666666666666667e-1 * t64926 * t2781 * t2785 - t116737 - 0.5895802469135802469e-2 * t116738 + t116741 - 0.49745833333333333332e-2 * t116743 + 0.23148148148148148149e-2 * t116745 - t116748 - 0.8041666666666666667e-2 * t32948 * t34148 - 0.8041666666666666667e-2 * t32921 * t34148;
    (t116747, t116753)
}
