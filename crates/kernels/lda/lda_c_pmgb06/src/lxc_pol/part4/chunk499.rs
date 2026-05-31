//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 499/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk499<F: Float>(t1981: F, t1983: F, t1910: F, t1914: F, t1918: F, t1922: F, t1927: F, t1930: F, t1932: F, t1935: F, t1937: F, t1938: F, t1939: F, t1959: F, t1965: F, t1971: F, t1974: F, t1976: F, t1979: F, t224: F) -> (F, F) {
    let t1985 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1981 * t1983;
    let t1986 = -t1910 - t1914 - t1918 + t1922 - t1927 + t1930 + t1932 + t1935 + t1937 - t1938 - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1939 - t1959 * t224 / F::cast_from(15.0_f64) + t1965 + t1971 + t1974 + t1976 + t1979 - t1985;
    (t1985, t1986)
}
