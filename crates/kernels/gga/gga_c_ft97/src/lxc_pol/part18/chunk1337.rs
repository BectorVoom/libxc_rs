//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1337/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1337<F: Float>(t105760: F, t105550: F, t446: F, t9073: F, t1882: F, t27106: F, t105376: F, t1969: F, t27103: F, t27100: F, t105741: F, t105744: F, t105746: F, t105751: F, t105754: F, t105757: F) -> (F, F, F, F, F, F) {
    let t105761 = 2.0 / 3.0 * t105760;
    let t105763 = t446 * t9073 * t105550;
    let t105765 = t1882 * t27106;
    let t105766 = 4.0 / 9.0 * t105765;
    let t105768 = t446 * t1969 * t105376;
    let t105770 = t1882 * t27103;
    let t105771 = 4.0 / 27.0 * t105770;
    let t105772 = t1882 * t27100;
    let t105773 = 4.0 / 9.0 * t105772;
    let t105774 = -t105741 - t105744 + 4.0 / 9.0 * t105746 - 3.0 / 8.0 * t105751 + 4.0 / 3.0 * t105754 - 4.0 / 3.0 * t105757 - t105761 - 4.0 / 3.0 * t105763 + t105766 + 2.0 * t105768 - t105771 + t105773;
    (t105763, t105765, t105768, t105770, t105772, t105774)
}
