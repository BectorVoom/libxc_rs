//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 470/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk470<F: Float>(t133: F, t1655: F, t1663: F, t1717: F, t1718: F, t1813: F, t1816: F, t1819: F, t1835: F, t1845: F, t1868: F, t1870: F, t1871: F, t1872: F) -> F {
    let t1878 = -t1655 + t1813 + t1663 + t1816 + t1819 - t1835 + t1717 + F::cast_from(0.5747516666666667_f64) * t1718 + F::cast_from(0.5747516666666667_f64) * t1868 + F::cast_from(5.172765_f64) * t1870 * t1871 * t1872 - F::cast_from(1.724255_f64) * t133 * t1845;
    t1878
}
