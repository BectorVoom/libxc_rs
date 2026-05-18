//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 521/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk521<F: Float>(t135: F, t1849: F, t1852: F, t1859: F, t1896: F, t1904: F, t1984: F, t1986: F, t1989: F, t1993: F, t1997: F, t2001: F, t2149: F, t2153: F, t2156: F, t273: F, t805: F) -> F {
    let t2159 = t135 * t2149 * t273 * t805 - t135 * t2153 * t2156 * t273 - t1849 + t1852 - t1859 + t1896 + t1904 + t1984 + t1986 - t1989 + t1993 - t1997 - t2001;
    t2159
}
