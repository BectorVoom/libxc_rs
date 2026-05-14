//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 458/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk458<F: Float>(t562: F, t597: F, t610: F, t1885: F, t1820: F, t1697: F, t219: F, t1413: F, t642: F, t639: F, t1764: F, t197: F, t1403: F, t590: F, t587: F, t720: F, t723: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1887 = t597 * t562 * t610;
    let t1888 = t1885 * t1887;
    let t1890 = 8.0 / 15.0 * t1820 * t1888;
    let t1891 = t219 * t1697;
    let t1892 = t1891 * t1413;
    let t1893 = t642 * t1892;
    let t1895 = 8.0 / 45.0 * t639 * t1893;
    let t1896 = t197 * t1764;
    let t1897 = t1896 * t1403;
    let t1898 = t590 * t1897;
    let t1900 = 8.0 / 45.0 * t587 * t1898;
    let t1902 = 4.0 / 9.0 * t720 * t723;
    (t1887, t1888, t1890, t1892, t1893, t1895, t1897, t1898, t1900, t1902)
}
