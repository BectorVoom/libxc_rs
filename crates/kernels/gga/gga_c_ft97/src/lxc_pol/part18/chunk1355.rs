//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1355/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1355<F: Float>(t105740: F, t105743: F, t105760: F, t105765: F, t105770: F, t105772: F, t105746: F, t105751: F, t105754: F, t105757: F, t105763: F, t105768: F, t105776: F, t105779: F, t105784: F, t105788: F, t105791: F, t105795: F, t105799: F, t105804: F, t95242: F, t95245: F, t95252: F, t95254: F) -> (F, F) {
    let t106087 = t105740 / 18.0;
    let t106088 = 2.0 / 9.0 * t105743;
    let t106093 = 2.0 / 9.0 * t105760;
    let t106095 = 4.0 / 27.0 * t105765;
    let t106097 = 4.0 / 81.0 * t105770;
    let t106098 = 4.0 / 27.0 * t105772;
    let t106099 = -t106087 - t106088 + 4.0 / 27.0 * t105746 - t105751 / 8.0 + 4.0 / 9.0 * t105754 - 4.0 / 9.0 * t105757 - t106093 - 4.0 / 9.0 * t105763 + t106095 + 2.0 / 3.0 * t105768 - t106097 + t106098;
    let t106111 = 2.0 / 27.0 * t105776 + 10.0 / 81.0 * t105779 - t105784 - 8.0 / 9.0 * t105788 + 8.0 / 27.0 * t105791 - t105795 / 4.0 + 4.0 / 3.0 * t105799 - 8.0 / 27.0 * t95242 + t105804 / 18.0 - 2.0 / 9.0 * t95245 - t95252 / 9.0 + t95254 / 3.0;
    (t106099, t106111)
}
