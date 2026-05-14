//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 878/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk878<F: Float>(t14795: F, t2061: F, t5512: F, t14639: F, t1686: F, t1852: F, t14650: F, t5592: F, t1840: F, t426: F, t474: F, t14584: F, t1856: F, t933: F, t1128: F, t1904: F, t281: F, t285: F) -> (F, F, F, F, F, F, F, F) {
    let t14796 = 2.93808 * t14795;
    let t14797 = t5512 * t2061;
    let t14802 = t1686 * t1852 * t14639;
    let t14803 = 5.87616 * t14802;
    let t14813 = t5592 * t14650;
    let t14814 = 11.75232 * t14813;
    let t14816 = t426 * t474 * t1840;
    let t14817 = 2.0 * t14816;
    let t14843 = t426 * t14584;
    let t14844 = 2.0 / 3.0 * t14843;
    let t14849 = t1686 * t1856 * t933;
    let t14850 = 1.46904 * t14849;
    let t14895 = t281 * t1904 * t1128 * t285;
    (t14796, t14797, t14803, t14814, t14817, t14844, t14850, t14895)
}
