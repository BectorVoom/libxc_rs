//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1091/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1091(t11750: f64, t452: f64, t11101: f64, t1926: f64, t1800: f64, t1971: f64, t2795: f64, t2758: f64, t309: f64, t1876: f64, t524: f64, t11096: f64, t11461: f64, t11491: f64, t11494: f64, t11960: f64, t1778: f64, t1805: f64, t1808: f64, t1823: f64, t1847: f64, t1856: f64, t1859: f64, t1907: f64, t1921: f64, t2114: f64, t2748: f64, t2752: f64, t2783: f64, t444: f64, t455: f64, t7494: f64, t7506: f64) -> f64 {
    let t11962 = t11750 * t452;
    let t11969 = t1926 * t11101;
    let t11970 = t11969 * t1800;
    let t11976 = t2795 * t1971;
    let t11983 = t2758 * t309;
    let t11986 = t1876 * t11101;
    let t11987 = t11986 * t1800;
    let t11991 = t524 * t11101;
    let t11994 = -21.324527244551554_f64 * t1847 * t11491 - 2.427516195194328_f64 * t11494 - 4.855032390388656_f64 * t1856 * t11096 + 2.427516195194328_f64 * t1778 * t2783 + t444 * t11960 + 19.489173774580152_f64 * t11962 * t455 + 3.7610742193750633_f64 * t1823 * t11461 + 1.8805371096875316_f64 * t2114 * t2748 - 18.635258017632964_f64 * t11970 - 1.8805371096875316_f64 * t7494 * t2752 - 1.8805371096875316_f64 * t1859 * t11461 + 19.489173774580152_f64 * t11976 * t455 - 19.489173774580152_f64 * t7506 * t2752 - 19.489173774580152_f64 * t1808 * t11461 - 0.04115066352984959_f64 * t11983 * t1921 - 19.489173774580152_f64 * t11987 + 0.04115066352984959_f64 * t11983 * t1907 - 18.635258017632964_f64 * t11991 * t1805;
    t11994
}
