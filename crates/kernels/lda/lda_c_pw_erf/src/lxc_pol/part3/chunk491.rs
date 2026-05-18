//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 491/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk491<F: Float>(t1982: F, t199: F, t568: F, t822: F, t1326: F, t1972: F, t519: F, t1608: F, t1611: F, t1615: F, t1623: F, t1922: F, t1923: F, t1927: F, t1929: F, t1931: F, t1934: F, t1936: F, t1937: F, t1939: F, t1962: F, t231: F) -> (F, F, F, F, F) {
    let t1984 = F::new(2.0) / F::new(15.0) * t1982 * t199;
    let t1985 = t822 * t568;
    let t1986 = F::new(4.0) / F::new(45.0) * t1985;
    let t1987 = t1326 * t1972;
    let t1989 = F::new(8.0) / F::new(45.0) * t519 * t1987;
    let t1990 = F::new(4.0) / F::new(3.0) * t1608 + t1611 - t1922 - t1923 + F::new(4.0) / F::new(3.0) * t1615 + t1623 + F::new(0.10821041362364843) * t1927 + F::new(4.0) / F::new(3.0) * t1929 + F::new(4.0) / F::new(3.0) * t1931 * t231 + F::new(4.0) / F::new(3.0) * t1934 + t1936 + t1937 + t1939 + t1962 + t1984 + t1986 - t1989;
    (t1984, t1986, t1987, t1989, t1990)
}
