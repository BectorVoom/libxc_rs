//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 824/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk824<F: Float>(t1044: F, t1642: F, t1413: F, t5522: F, t639: F, t2584: F, t5125: F, t1820: F, t2666: F, t5137: F, t2673: F, t4934: F, t219: F, t5480: F, t2679: F, t1027: F, t1724: F) -> (F, F, F, F, F, F) {
    let t7863 = t1044 * t1642;
    let t7864 = t7863 * t1413;
    let t7865 = t5522 * t7864;
    let t7867 = 4.0 / 27.0 * t639 * t7865;
    let t7868 = t5125 * t2584;
    let t7870 = 32.0 / 135.0 * t1820 * t7868;
    let t7871 = t5137 * t2666;
    let t7873 = 16.0 / 135.0 * t639 * t7871;
    let t7874 = t4934 * t2673;
    let t7876 = 32.0 / 135.0 * t639 * t7874;
    let t7877 = t5480 * t219;
    let t7878 = t7877 * t2679;
    let t7880 = 16.0 / 81.0 * t639 * t7878;
    let t7881 = t1027 * t1724;
    (t7867, t7870, t7873, t7876, t7880, t7881)
}
