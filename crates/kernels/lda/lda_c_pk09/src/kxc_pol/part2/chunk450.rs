//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 450/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk450<F: Float>(t2674: F, t93: F, t1372: F, t1374: F, t1376: F, t1378: F, t2502: F, t2505: F, t2542: F, t2546: F, t1381: F, t306: F, t1388: F, t1462: F, t1464: F, t1466: F, t1474: F, t1476: F, t1481: F, t1489: F, t1490: F, t1521: F, t1527: F, t1529: F, t1531: F, t2513: F, t2517: F, t2521: F, t311: F) -> (F, F, F, F) {
    let t2675 = t93 * t2674;
    let t2688 = t1372 - 6.25 * t2542 + t1374 + 6.25 * t2546 + t1376 - 1.2466946262544771 * t2502 + t1378 + 1.2466946262544771 * t2505;
    let t2689 = t2688 * t1381;
    let t2690 = t2689 * t306;
    let t2693 = -t1462 - t1464 + t1466 - 1.7770439370459628 * t1388 * t2675 - 7.108175748183851 * t1476 * t2517 + 7.108175748183851 * t1481 * t2521 + 2.427516195194328 * t1490 * t2513 - 2.427516195194328 * t2690 * t311 + t1474 + t1489 + t1521 + t1527 - t1529 + t1531;
    (t2688, t2689, t2690, t2693)
}
