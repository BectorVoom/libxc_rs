//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 434/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk434<F: Float>(t1526: F, t1619: F, t1532: F, t473: F, t1536: F, t103: F, t1523: F, t1528: F, t1534: F, t1538: F, t1607: F, t1614: F, t1615: F) -> (F, F, F, F) {
    let t1620 = t1619 * t1526;
    let t1623 = t473 * t1532;
    let t1626 = t473 * t1536;
    let t1629 = t1607 + F::cast_from(0.023994444444444443_f64) * t1523 - F::cast_from(0.023994444444444443_f64) * t1528 + F::cast_from(0.07198333333333333_f64) * t1534 - F::cast_from(0.035991666666666665_f64) * t1538 + t1614 + F::cast_from(0.008888888888888889_f64) * t1615 - F::cast_from(0.0022222222222222222_f64) * t103 * t1620 + F::cast_from(0.013333333333333334_f64) * t103 * t1623 - F::cast_from(0.006666666666666667_f64) * t103 * t1626;
    (t1620, t1623, t1626, t1629)
}
