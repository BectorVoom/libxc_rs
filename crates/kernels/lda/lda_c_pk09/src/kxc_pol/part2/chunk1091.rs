//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1091/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1091<F: Float>(t11750: F, t452: F, t11101: F, t1926: F, t1800: F, t1971: F, t2795: F, t2758: F, t309: F, t1876: F, t524: F, t11096: F, t11461: F, t11491: F, t11494: F, t11960: F, t1778: F, t1805: F, t1808: F, t1823: F, t1847: F, t1856: F, t1859: F, t1907: F, t1921: F, t2114: F, t2748: F, t2752: F, t2783: F, t444: F, t455: F, t7494: F, t7506: F) -> F {
    let t11962 = t11750 * t452;
    let t11969 = t1926 * t11101;
    let t11970 = t11969 * t1800;
    let t11976 = t2795 * t1971;
    let t11983 = t2758 * t309;
    let t11986 = t1876 * t11101;
    let t11987 = t11986 * t1800;
    let t11991 = t524 * t11101;
    let t11994 = -F::new(21.324527244551554) * t1847 * t11491 - F::new(2.427516195194328) * t11494 - F::new(4.855032390388656) * t1856 * t11096 + F::new(2.427516195194328) * t1778 * t2783 + t444 * t11960 + F::new(19.489173774580152) * t11962 * t455 + F::new(3.7610742193750633) * t1823 * t11461 + F::new(1.8805371096875316) * t2114 * t2748 - F::new(18.635258017632964) * t11970 - F::new(1.8805371096875316) * t7494 * t2752 - F::new(1.8805371096875316) * t1859 * t11461 + F::new(19.489173774580152) * t11976 * t455 - F::new(19.489173774580152) * t7506 * t2752 - F::new(19.489173774580152) * t1808 * t11461 - F::new(0.04115066352984959) * t11983 * t1921 - F::new(19.489173774580152) * t11987 + F::new(0.04115066352984959) * t11983 * t1907 - F::new(18.635258017632964) * t11991 * t1805;
    t11994
}
