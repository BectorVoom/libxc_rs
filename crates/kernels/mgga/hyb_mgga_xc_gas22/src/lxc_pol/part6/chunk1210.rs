//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1210/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1210<F: Float>(t3984: F, t763: F, t10159: F, t2014: F, t684: F, t10505: F, t10509: F, t3188: F, t10322: F, t1890: F, t10317: F, t10306: F, t10310: F, t7942: F, t10293: F, t10304: F, t10315: F, t10320: F, t10325: F, t2002: F, t2022: F, t20225: F, t2028: F, t2054: F, t3171: F, t3177: F, t3925: F, t3938: F, t572: F, t6291: F, t675: F, t8296: F) -> (F, F, F, F, F, F) {
    let t28136 = t763 * t3984;
    let t28150 = t684 * t2014 * t10159;
    let t28153 = t684 * t2014 * t10505;
    let t28156 = t684 * t2014 * t10509;
    let t28162 = t3188 * t3188;
    let t28183 = t1890 * t10322;
    let t28185 = t1890 * t10317;
    let t28187 = t1890 * t10306;
    let t28189 = t7942 * t10310;
    let t28223 = t572 * t3177 * t10320 * t2002 / 27.0 - 2.0 / 81.0 * t28183 + 2.0 / 243.0 * t28185 + 2.0 / 27.0 * t28187 + 44.0 / 81.0 * t28189 - t572 * t3177 * t10304 * t2002 / 9.0 - 2.0 / 81.0 * t572 * t3171 * t2054 * t10325 * t675 - t572 * t3171 * t10315 * t2002 / 81.0 - 5.0 / 243.0 * t572 * t8296 * t6291 * t3938 * t2028 + 2.0 / 27.0 * t572 * t3177 * t2022 * t10325 * t675 + 4.0 / 9.0 * t572 * t3177 * t10293 * t2028 + 20.0 / 81.0 * t572 * t8296 * t20225 * t3925 * t2028;
    (t28136, t28150, t28153, t28156, t28162, t28223)
}
