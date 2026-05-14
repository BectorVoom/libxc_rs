//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1066/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1066<F: Float>(t10928: F, t10930: F, t10935: F, t10939: F, t10942: F, t10946: F, t10950: F, t7035: F, t7037: F, t9159: F, t9217: F, t9218: F, t11056: F, t987: F, t10878: F, t10880: F, t10882: F, t10884: F, t10886: F, t10956: F, t10958: F, t1434: F, t2533: F, t4284: F, t4297: F, t4300: F, t7140: F, t7159: F, t9199: F, t979: F) -> (F, F, F) {
    let t11066 = 0.31558125e0 * t10928 + 0.6311625e0 * t10930 - t7035 + 0.34731666666666666666e0 * t7037 + 0.69463333333333333333e0 * t9159 - t9217 - t9218 - 0.20839e0 * t10935 + 0.62517e0 * t10939 - 0.20839e0 * t10942 + 0.312585e0 * t10946 + 0.312585e0 * t10950;
    let t11067 = t11056 + t11066;
    let t11068 = t11067 * t987;
    let t11075 = -2.0 * t7140 * t4284 + 1.0 * t2533 * t4297 + 1.0 * t979 * t11068 + 0.32163958997385070134e2 * t7159 * t4300 - t10878 - t10880 - t10882 + t10884 - t10886 - t10956 - t10958 + 0.11696447245269292414e1 * t9199 * t1434;
    (t11067, t11068, t11075)
}
