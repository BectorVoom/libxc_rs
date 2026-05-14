//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1362/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1362<F: Float>(t10010: F, t10359: F, t1592: F, t2122: F, t2124: F, t2184: F, t24100: F, t2654: F, t27661: F, t29054: F, t29449: F, t29452: F, t29455: F, t29457: F, t3092: F, t360: F, t481: F, t551: F, t552: F, t560: F, t7313: F, t7512: F, t8240: F, t8792: F, t8804: F, t8825: F, t9100: F, t9105: F, t9152: F, t9509: F, t9521: F) -> (F,) {
    let t33345 = 0.7801399566048841707e0 * t24100 * t3092 + 0.2600466522016280569e0 * t7313 * t9100 + 0.39006997830244208535e0 * t8240 * t9105 + 0.86682217400542685632e-1 * t2184 * t551 * t552 * t10359 * t560 + 0.13002332610081402845e0 * t1592 * t551 * t552 * t10359 * t481 - 0.38415120233790484326e0 * t29449 - 0.7801399566048841707e0 * t7512 * t360 * t8825 * t2654 - 0.13002332610081402845e0 * t8792 * t9152 - 0.15602799132097683414e1 * t29054 * t8804 + 0.38087975358139160776e-1 * t29452 + 0.11426392607441748233e0 * t29455 + 0.17348729279022588207e-2 * t29457 + 0.52009330440325611378e0 * t9521 * t9509 + 0.16463622957338778996e0 * t2122 * t2124 * t27661 * t10010;
    (t33345,)
}
