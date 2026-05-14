//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 747/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk747<F: Float>(t119: F, t2418: F, t3254: F, t7731: F, t155: F, t7991: F, t151: F, t8141: F, t1062: F, t2238: F, t721: F, t1067: F, t2271: F, t192: F, t2214: F, t2314: F, t3753: F, t4411: F, t4660: F, t709: F, t713: F, t7706: F, t7727: F, t7776: F) -> (F,) {
    let t8555 = t2418 * t119;
    let t8560 = t3254 * t7731;
    let t8564 = t155 * t7991;
    let t8566 = t151 * t8141;
    let t8570 = t2238 * t1062;
    let t8571 = t8570 * t721;
    let t8573 = t2271 * t1067;
    let t8575 = 2.427516195194328 * t3753 * t2214 + 2.2140749178833072 * t192 * t7776 + 2.2140749178833072 * t192 * t7706 - 1.8805371096875316 * t8555 * t713 - 1.8805371096875316 * t8555 * t709 + 19.489173774580152 * t8560 + 2.2140749178833072 * t7727 * t713 + 12.992782516386768 * t8564 + 1.2536914064583544 * t8566 - t4660 * t2314 + 14.71989892086604 * t4411 + 2.2140749178833072 * t8571 - 3.2915558116322368 * t8573;
    (t8575,)
}
