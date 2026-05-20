//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1377/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1377<F: Float>(t13180: F, t493: F, t225: F, t13038: F, t42859: F, t460: F, t13045: F, t43351: F, t44531: F, t44535: F, t1209: F, t17845: F) -> (F, F, F, F, F, F) {
    let t45551 = F::new(1.0) / t13180 / t493;
    let t45552 = t225 * t45551;
    let t45607 = t42859 * t13038;
    let t45608 = t460 * t45607;
    let t45610 = t43351 * t13045;
    let t45618 = t42859 * t44531;
    let t45619 = t460 * t45618;
    let t45620 = t43351 * t44535;
    let t45654 = t1209 * t17845;
    (t45552, t45608, t45610, t45619, t45620, t45654)
}
