//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1384/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1384<F: Float>(t33521: F, t9442: F, t10500: F, t20621: F, t3480: F, t467: F, t6174: F, t9452: F, t110687: F, t110692: F, t110695: F, t110697: F, t110699: F, t110702: F, t110704: F, t20026: F, t2718: F, t32015: F, t32087: F, t32111: F, t32142: F, t32189: F, t33346: F, t33373: F, t6221: F) -> (F, F) {
    let t114517 = 0.69444444444444444446e-2 * t33521 * t9442;
    let t114520 = t10500 * t3480 * t467 * t20621;
    let t114531 = t6174 * t9452;
    let t114543 = -t114517 + 0.33163888888888888888e-2 * t114520 - 0.10185185185185185186e0 * t6221 * t32111 * t2718 - 0.10416666666666666667e-1 * t6221 * t32142 * t2718 - 0.71481481481481481483e-2 * t110687 - 0.20833333333333333334e-1 * t33373 * t32015 + 0.92592592592592592594e-2 * t32087 * t114531 * t20026 - 0.46296296296296296298e-2 * t110692 - 0.17870370370370370371e-2 * t110695 + 0.69444444444444444446e-2 * t110697 + 0.69444444444444444446e-2 * t110699 + 0.34722222222222222223e-2 * t110702 + 0.69444444444444444446e-2 * t110704 - 0.21444444444444444446e-1 * t32189 * t33346;
    (t114520, t114543)
}
