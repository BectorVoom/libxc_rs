//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1105/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1105<F: Float>(t2288: F, t4262: F, t7450: F, t922: F, t2310: F, t7780: F, t31643: F, t527: F, t2001: F, t5255: F, t5003: F, t1418: F, t7605: F) -> (F, F, F, F, F, F) {
    let t35660 = t7450 * t4262 * t2288 * t922;
    let t35662 = t7780 * t2310;
    let t35664 = t31643 * t527;
    let t35668 = t2001 * t5255;
    let t35670 = t2001 * t5003;
    let t35672 = t7605 * t1418;
    (t35660, t35662, t35664, t35668, t35670, t35672)
}
