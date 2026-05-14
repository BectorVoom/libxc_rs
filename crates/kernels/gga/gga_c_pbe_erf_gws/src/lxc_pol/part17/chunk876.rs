//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 876/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk876<F: Float>(t2932: F, t751: F, t2936: F, t2033: F, t2922: F, t2986: F, t5657: F, t5888: F, t6036: F, t6039: F, t6043: F, t6049: F, t6050: F, t6053: F, t6058: F, t6059: F, t6061: F, t6064: F, t988: F) -> (F,) {
    let t8502 = 0.39914113367515363646e-1 * t751 * t2932;
    let t8503 = t751 * t2936;
    let t8514 = -t6036 + t8502 + 0.39914113367515363646e-1 * t8503 - 0.36437153863430196886e-4 * t6039 - t6043 + t6049 - 0.10643763564670763639e0 * t6050 - t6053 - t6058 + 0.19957056683757681823e-1 * t6059 + 0.79828226735030727292e-1 * t6061 + t6064 + t988 * t5888 - 2.0 * t2922 * t2033 + 3.0 * t2986 * t5657;
    (t8514,)
}
