//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 384/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk384<F: Float>(t1754: F, t1765: F, t1684: F, t1735: F, t1732: F, t1738: F, t1762: F, t1769: F, t495: F, t452: F, t337: F, t447: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1862 = F::cast_from(3.2084841915276807_f64) * t1754;
    let t1864 = F::cast_from(1.0694947305092268_f64) * t1765;
    let t1866 = F::new(0.64) * t1684;
    let t1868 = F::cast_from(0.21333333333333335_f64) * t1735;
    let t1870 = t1862 - F::cast_from(3.2084841915276807_f64) * t1762 + t1864 + F::cast_from(3.2084841915276807_f64) * t1769 + t1866 - F::new(0.64) * t1732 + t1868 + F::new(0.64) * t1738;
    let t1871 = F::new(1.0) / t495;
    let t1872 = t1870 * t1871;
    let t1873 = t1872 * t452;
    let t1876 = t447 * t337;
    (t1862, t1864, t1866, t1868, t1870, t1871, t1872, t1873, t1876)
}
