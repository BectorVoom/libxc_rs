//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 876/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk876<F: Float>(t2615: F, t2632: F, t3443: F, t597: F, t562: F, t1885: F, t1820: F, t3534: F, t5018: F, t1017: F, t7468: F, t7467: F, t1627: F, t3500: F, t1648: F, t3504: F) -> (F, F, F, F, F, F) {
    let t10907 = 8.0 / 15.0 * t2615 * t2632;
    let t10908 = t597 * t3443;
    let t10909 = t10908 * t562;
    let t10910 = t1885 * t10909;
    let t10912 = 4.0 / 15.0 * t1820 * t10910;
    let t10913 = t5018 * t3534;
    let t10914 = t1820 * t10913;
    let t10915 = 16.0 / 45.0 * t10914;
    let t10916 = t7468 * t1017;
    let t10917 = t7467 * t10916;
    let t10919 = 8.0 / 15.0 * t1820 * t10917;
    let t10921 = 8.0 / 45.0 * t1627 * t3500;
    let t10923 = 8.0 / 45.0 * t1648 * t3504;
    (t10907, t10912, t10915, t10919, t10921, t10923)
}
