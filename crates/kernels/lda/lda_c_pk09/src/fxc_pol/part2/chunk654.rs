//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 654/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk654<F: Float>(t307: F, t5759: F, t1615: F, t5569: F, t1435: F, t1562: F, t1568: F, t1633: F, t280: F, t1632: F, t5408: F, t1614: F) -> (F, F, F, F, F, F, F, F) {
    let t5760 = t307 * t5759;
    let t5762 = t1615 * t5569;
    let t5773 = t1562 * t1435;
    let t5775 = t1568 * t1435;
    let t5777 = t1633 * t280;
    let t5778 = t1632 * t5777;
    let t5783 = F::cast_from(0.037002892246025966_f64) * t5408;
    let t5785 = t1614 * t280;
    (t5760, t5762, t5773, t5775, t5777, t5778, t5783, t5785)
}
