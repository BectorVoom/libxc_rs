//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 467/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk467<F: Float>(t562: F, t597: F, t610: F, t1885: F, t1820: F, t1697: F, t219: F, t1413: F, t642: F, t639: F, t1764: F, t197: F) -> (F, F, F, F, F, F, F) {
    let t1887 = t597 * t562 * t610;
    let t1888 = t1885 * t1887;
    let t1890 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1820 * t1888;
    let t1891 = t219 * t1697;
    let t1892 = t1891 * t1413;
    let t1893 = t642 * t1892;
    let t1895 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t639 * t1893;
    let t1896 = t197 * t1764;
    (t1887, t1888, t1890, t1892, t1893, t1895, t1896)
}
