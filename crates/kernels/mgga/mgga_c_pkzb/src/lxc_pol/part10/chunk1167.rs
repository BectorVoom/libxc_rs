//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1167/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1167<F: Float>(t154: F, t18060: F, t276: F, t655: F, t1843: F, t5688: F, t5690: F, t735: F, t486: F, t779: F, t1885: F, t148: F, t179: F, t299: F, t5672: F, t2045: F, t2057: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18063 = t276 * t154 * t18060 * t655;
    let t18067 = t276 * t154 * t5688 * t1843;
    let t18084 = t735 * t5690;
    let t18086 = t486 * t779;
    let t18089 = t276 * t154 * t18086 * t1885;
    let t18107 = t148 * t779;
    let t18110 = t299 * t179 * t18107 * t655;
    let t18114 = t299 * t179 * t5672 * t1843;
    let t18123 = t2057 * t2045;
    (t18063, t18067, t18084, t18086, t18089, t18107, t18110, t18114, t18123)
}
