//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 384/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk384(t1754: f64, t1765: f64, t1684: f64, t1735: f64, t1732: f64, t1738: f64, t1762: f64, t1769: f64, t495: f64, t452: f64, t337: f64, t447: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1862 = 3.2084841915276807_f64 * t1754;
    let t1864 = 1.0694947305092268_f64 * t1765;
    let t1866 = 0.64_f64 * t1684;
    let t1868 = 0.21333333333333335_f64 * t1735;
    let t1870 = t1862 - 3.2084841915276807_f64 * t1762 + t1864 + 3.2084841915276807_f64 * t1769 + t1866 - 0.64_f64 * t1732 + t1868 + 0.64_f64 * t1738;
    let t1871 = 1.0_f64 / t495;
    let t1872 = t1870 * t1871;
    let t1873 = t1872 * t452;
    let t1876 = t447 * t337;
    (t1862, t1864, t1866, t1868, t1870, t1871, t1872, t1873, t1876)
}
