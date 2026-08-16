//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1202/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1202(t21750: f64, t21768: f64, t257: f64, t11083: f64, t14971: f64, t14973: f64, t14977: f64, t283: f64, t8598: f64, t8603: f64, t8610: f64, t8612: f64, t8621: f64, t8626: f64, t8629: f64, t8633: f64, t8637: f64, t8640: f64, t8644: f64) -> (f64, f64) {
    let t21770 = (t21750 + t21768) * t257;
    let t21774 = -t8598 + t8603 + t8610 - t8612 - t11083 - 0.0005493434191801964_f64 * t14971 + 0.0007324578922402618_f64 * t14973 + 0.0197516734986138_f64 * t21770 * t283 + t8621 - t8626 + 3.0_f64 * t14977 - t8629 - t8633 - t8637 + t8640 + t8644;
    (t21770, t21774)
}
