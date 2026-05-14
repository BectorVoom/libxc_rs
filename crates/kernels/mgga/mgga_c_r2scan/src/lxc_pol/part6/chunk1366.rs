//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1366/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1366<F: Float>(t22731: F, t8161: F, t25684: F, t6086: F, t6535: F, t2155: F, t8088: F, t537: F, t7088: F, t6500: F, t7383: F, t1592: F, t20499: F, t20989: F, t20991: F, t2122: F, t2124: F, t22721: F, t24188: F, t24253: F, t24268: F, t25275: F, t4933: F, t495: F, t5109: F, t551: F, t552: F, t6106: F, t6293: F, t7321: F, t7337: F, t7583: F, t8022: F, t8035: F, t938: F) -> (F,) {
    let t25898 = t22731 * t8161;
    let t25901 = t6535 * t6086 * t25684;
    let t25904 = t2155 * t8088 * t25684;
    let t25911 = t537 * t7088;
    let t25916 = t7383 * t6500;
    let t25934 = 0.78013995660488417067e0 * t8022 * t8035 - 0.24451668256642615404e1 * t20989 - 0.17563392970889009433e0 * t20991 + 0.1047928639570397803e0 * t25898 + 0.69861909304693186866e-1 * t25901 + 0.58544643236296698111e-1 * t25904 + 0.13002332610081402845e0 * t1592 * t551 * t552 * t938 * t4933 + 0.16463622957338778996e0 * t2122 * t2124 * t25911 * t495 - 0.41607464352260489103e1 * t25916 - 0.15602799132097683414e1 * t6106 * t5109 * t25275 + 0.31205598264195366828e1 * t20499 * t5109 * t24253 - 0.65854491829355115984e0 * t2122 * t7337 * t24188 - 0.9878173774403267398e0 * t6293 * t7321 * t24268 + 0.78013995660488417068e0 * t22721 * t5109 * t7583 * t495;
    (t25934,)
}
