//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1849/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1849<F: Float>(t1458: F, t1914: F, t1921: F, t25049: F, t25072: F, t3: F, t575: F, t6937: F, t6951: F, t75808: F, t86897: F, t86903: F, t86909: F, t92517: F, t92552: F) -> F {
    let tv4rho44 = t3 * t575 * t92517 + t1458 * t92552 + F::new(4.0) * t1914 * t25072 + F::new(4.0) * t1921 * t25049 + F::new(6.0) * t6937 * t6951 + F::new(4.0) * t75808 + F::new(12.0) * t86897 + F::new(12.0) * t86903 + F::new(4.0) * t86909;
    tv4rho44
}
