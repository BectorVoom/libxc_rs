//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2517/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2517<F: Float>(t225: F, t45551: F, t1209: F, t13107: F, t460: F, t13038: F, t42859: F, t44531: F, t473: F, t17879: F, t17845: F, t17852: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t45552 = t225 * t45551;
    let t45568 = t1209 * t13107;
    let t45575 = t460 * t13107;
    let t45607 = t42859 * t13038;
    let t45608 = t460 * t45607;
    let t45618 = t42859 * t44531;
    let t45619 = t460 * t45618;
    let t45624 = t473 * t13107;
    let t45634 = t1209 * t17879;
    let t45654 = t1209 * t17845;
    let t45659 = t1209 * t17852;
    (t45552, t45568, t45575, t45607, t45608, t45618, t45619, t45624, t45634, t45654, t45659)
}
