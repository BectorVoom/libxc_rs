//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 682/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk682<F: Float>(t6680: F, t757: F, t188: F, t1917: F, t732: F, t1916: F, t1955: F, t1912: F, t2048: F, t559: F, t592: F, t1956: F, t6647: F, t6648: F, t6675: F, t737: F) -> (F, F, F, F, F) {
    let t6681 = t6680 * t757;
    let t6682 = t188 * t6681;
    let t6684 = t732 * t1917;
    let t6686 = t1916 * t1955;
    let t6687 = t188 * t6686;
    let t6689 = t732 * t1912;
    let t6693 = t2048 * t559;
    let t6694 = 96.0 * t6693;
    let t6695 = t2048 * t592;
    let t6696 = 96.0 * t6695;
    let t6697 = -t6647 + 3.0 / 2.0 * t6648 + t188 * t6675 / 2.0 + 3.0 / 2.0 * t737 * t1956 + 35.0 / 3.0 * t6682 - 7.0 * t6684 - 7.0 / 2.0 * t6687 + 3.0 / 2.0 * t6689 + 3.0 / 2.0 * t737 * t1912 - t6694 - t6696;
    (t6681, t6686, t6694, t6696, t6697)
}
