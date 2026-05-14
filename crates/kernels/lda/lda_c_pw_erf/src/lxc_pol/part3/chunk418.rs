//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 418/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk418<F: Float>(t1638: F, t1639: F, t1399: F, t1407: F, t1445: F, t1448: F, t1452: F, t1456: F, t1464: F, t1471: F, t1629: F, t1632: F, t1637: F, t1474: F, t1478: F, t1482: F, t1490: F, t1500: F, t1510: F, t1515: F, t1517: F, t1521: F, t1526: F, t1528: F, t1531: F) -> (F, F, F) {
    let t1641 = 0.011181742741110338 * t1638 * t1639;
    let t1642 = t1399 + t1407 + t1629 + 0.21642082724729686 * t1632 + t1637 + t1641 - t1445 + t1448 + t1452 + t1456 + t1464 - t1471;
    let t1643 = t1474 + t1478 + t1482 + t1490 + t1500 + t1510 + t1515 + t1517 - t1521 + t1526 + t1528 - t1531;
    (t1641, t1642, t1643)
}
