//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1314/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1314<F: Float>(t113721: F, t1390: F, t2173: F, t5626: F, t113727: F, t3532: F, t20160: F, t34692: F, t32102: F, t110463: F, t110566: F, t110635: F, t110655: F, t113914: F, t118891: F, t26036: F, t32008: F, t32026: F, t32066: F, t32087: F, t32189: F, t33384: F, t33477: F, t34693: F, t34697: F, t34744: F) -> (F, F, F) {
    let t118898 = t113721 * t2173 * t1390 * t5626;
    let t118905 = t113727 * t2173 * t3532 * t5626;
    let t118919 = t20160 * t34692;
    let t118920 = t32102 * t118919;
    let t118930 = -0.53611111111111111112e-2 * t32008 * t118898 - 0.15520416666666666667e-2 * t110635 * t118891 + 0.35740740740740740741e-2 * t32008 * t118905 + 0.92592592592592592594e-2 * t32087 * t118905 - 0.13888888888888888889e-1 * t32087 * t113914 * t26036 - 0.23280625000000000001e-2 * t110566 * t34693 - 0.10722222222222222223e-1 * t32189 * t34697 - 0.23280625000000000001e-2 * t110463 * t34693 - 0.77602083333333333337e-3 * t118920 + 0.62081666666666666669e-2 * t110655 * t34693 - 0.13888888888888888889e-1 * t33384 * t33477 - 0.8041666666666666667e-2 * t32026 * t34744 - 0.8041666666666666667e-2 * t32066 * t34744;
    (t118898, t118919, t118930)
}
