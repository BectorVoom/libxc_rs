//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1202/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1202<F: Float>(t21750: F, t21768: F, t257: F, t11083: F, t14971: F, t14973: F, t14977: F, t283: F, t8598: F, t8603: F, t8610: F, t8612: F, t8621: F, t8626: F, t8629: F, t8633: F, t8637: F, t8640: F, t8644: F) -> (F, F) {
    let t21770 = (t21750 + t21768) * t257;
    let t21774 = -t8598 + t8603 + t8610 - t8612 - t11083 - F::cast_from(0.0005493434191801964_f64) * t14971 + F::cast_from(0.0007324578922402618_f64) * t14973 + F::cast_from(0.0197516734986138_f64) * t21770 * t283 + t8621 - t8626 + F::new(3.0) * t14977 - t8629 - t8633 - t8637 + t8640 + t8644;
    (t21770, t21774)
}
