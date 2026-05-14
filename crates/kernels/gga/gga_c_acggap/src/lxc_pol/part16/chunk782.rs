//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 782/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk782<F: Float>(t7884: F, t7911: F, t7887: F, t7930: F, t862: F, t309: F, t871: F, t620: F, t2122: F, t310: F, t441: F, t7923: F, t621: F, t394: F, t2130: F, t2149: F) -> (F, F, F, F, F, F, F, F) {
    let t29976 = t7884 * t7911;
    let t29977 = t29976 * t7887;
    let t29979 = t862 * t7930;
    let t29980 = t871 * t309;
    let t29982 = t29979 * t620 * t29980;
    let t29988 = t310 * t2122;
    let t29991 = t7923 * t441;
    let t29992 = t29991 * t621;
    let t29997 = t394 * t2122;
    let t30005 = t7923 * t2130;
    let t30006 = t30005 * t2149;
    (t29977, t29979, t29982, t29988, t29992, t29997, t30005, t30006)
}
