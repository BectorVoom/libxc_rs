//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 787/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk787<F: Float>(t1932: F, t704: F, t1940: F, t702: F, t1971: F, t723: F, t1979: F, t721: F, t1915: F, t690: F, t5831: F, t703: F, t1954: F, t709: F, t5484: F, t722: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5883 = t704 * t1932;
    let t5887 = t1932 * t1940 * t702;
    let t5890 = t723 * t1971;
    let t5893 = t1971 * t1979;
    let t5894 = t5893 * t721;
    let t5897 = t690 * t1915;
    let t5900 = t5831 * t703;
    let t5903 = t709 * t1954;
    let t5906 = t5484 * t722;
    (t5883, t5887, t5890, t5893, t5894, t5897, t5900, t5903, t5906)
}
