//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1254/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1254<F: Float>(t322: F, t23519: F, t1022: F, t1024: F, t1026: F, t1310: F, t1312: F, t2412: F, t2414: F, t2418: F, t2422: F, t2426: F, t6709: F, t6715: F, t839: F, t8467: F, t8469: F, t8471: F, t8473: F) -> (F, F) {
    let t332 = 0.25e1 < t322;
    let t23556 = piecewise3(t332, 0.0, t23519);
    let t23594 = -0.64e0 * t23556 - 0.22988522834472e3 * t1022 * t6715 + 0.18607840861392e3 * t1024 * t6715 - 0.4355305902528e2 * t1026 * t6715 + 0.2204323381566e3 * t2414 * t1312 - 0.34482784251708e3 * t2418 * t1312 + 0.18607840861392e3 * t2422 * t1312 - 0.3266479426896e2 * t2426 * t1312 - 0.27642340881882e2 * t2412 * t1310 - 0.27642340881882e2 * t8467 * t839 - 0.27642340881882e2 * t2414 * t1310 - 0.9214113627294e1 * t1022 * t6709 + 0.1102161690783e3 * t8469 * t839 + 0.1102161690783e3 * t2418 * t1310 + 0.367387230261e2 * t1024 * t6709 - 0.11494261417236e3 * t8471 * t839 - 0.11494261417236e3 * t2422 * t1310 - 0.3831420472412e2 * t1026 * t6709 + 0.4651960215348e2 * t8473 * t839;
    (t23556, t23594)
}
