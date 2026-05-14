//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 697/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk697<F: Float>(t6878: F, t6879: F, t161: F, t2024: F, t127: F, t136: F, t2079: F, t634: F, t648: F, t108: F, t6567: F, t117: F, t56: F, t2021: F, t3439: F, t636: F, t6778: F, t6782: F, t6787: F, t6792: F, t6795: F, t6797: F, t6800: F, t6804: F, t6870: F, t6873: F, t6876: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6880 = t6878 * t6879;
    let t6881 = t161 * t6880;
    let t6884 = t6878 * t2024;
    let t6885 = t161 * t6884;
    let t6888 = t6878 * t127;
    let t6889 = t161 * t6888;
    let t6892 = t2079 * t136;
    let t6893 = t634 * t6892;
    let t6894 = t6893 * t648;
    let t6896 = t108 * t6567;
    let t6899 = 455.0 / 1296.0 * t6896 * t56 * t117;
    let t6900 = -0.81498388966769604888e-2 * t636 * t6778 + 0.32599355586707841954e-1 * t636 * t6782 - 0.65198711173415683909e-1 * t2021 * t6787 + 0.16299677793353920977e-1 * t2021 * t6792 - 0.30426065214260652491e0 * t6795 + 0.38032581517825815615e-1 * t6797 - 0.7606516303565163123e-1 * t6800 + 0.16299677793353920977e0 * t3439 * t6804 - 0.27166129655589868296e-2 * t636 * t6870 + 0.38032581517825815615e-1 * t6873 - 0.16299677793353920978e-1 * t6876 * t6881 + 0.16299677793353920977e-1 * t2021 * t6885 - 0.27166129655589868296e-2 * t636 * t6889 - 0.21551796193434628848e0 * t6894 - t6899;
    (t6880, t6881, t6884, t6885, t6888, t6889, t6892, t6893, t6896, t6900)
}
