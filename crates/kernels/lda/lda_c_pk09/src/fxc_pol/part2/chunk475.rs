//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 475/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk475<F: Float>(t1755: F, t1766: F, t1771: F, t1773: F, t2733: F, t2736: F, t2803: F, t2807: F, t1776: F, t452: F, t2730: F, t537: F, t545: F, t1842: F, t1847: F, t1856: F, t1954: F, t1956: F, t1958: F, t1994: F, t2002: F, t2019: F, t2023: F, t2025: F, t2744: F, t2748: F, t2752: F, t455: F) -> (F, F, F, F, F, F) {
    let t2938 = t1755 - 6.25 * t2803 + t1766 + 6.25 * t2807 + t1771 - 1.2466946262544771 * t2733 + t1773 + 1.2466946262544771 * t2736;
    let t2939 = t2938 * t1776;
    let t2940 = t2939 * t452;
    let t2943 = t537 * t2730;
    let t2946 = t545 * t2730;
    let t2949 = -7.108175748183851 * t1842 * t2744 + 7.108175748183851 * t1847 * t2748 + 2.427516195194328 * t1856 * t2752 - 2.427516195194328 * t2940 * t455 - t1954 - t1956 + t1958 + t1994 + t2002 + t2019 + t2023 + t2025 - 2.2140749178833072 * t2943 * t455 - 2.427516195194328 * t2946 * t455;
    (t2938, t2939, t2940, t2943, t2946, t2949)
}
