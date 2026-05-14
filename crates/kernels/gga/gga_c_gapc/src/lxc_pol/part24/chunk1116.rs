//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1116/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1116<F: Float>(t10538: F, t30523: F, t12433: F, t1611: F, t1112: F, t30867: F, t13281: F, t1617: F, t3859: F, t34344: F, t34346: F, t34351: F, t34353: F, t34356: F, t34359: F, t34361: F, t34364: F, t34367: F, t34370: F, t34373: F) -> (F, F, F, F, F) {
    let t38082 = 12.0 * t30523 * t10538;
    let t38086 = 2.0 * t1611 * t12433;
    let t38088 = 2.0 * t30867 * t1112;
    let t38093 = 24.0 * t13281 * t3859 * t1617;
    let t38118 = 0.2318836277704281739e-4 * t34344 + 0.86880925264517213544e-4 * t34346 - 0.10136107947527008247e-2 * t34351 - 0.80966145833333333339e-4 * t34353 + 0.11584123368602295139e-4 * t34356 + 0.11584123368602295139e-4 * t34359 - 0.11382560960801989336e-6 * t34361 + 0.2023819338830593704e-6 * t34364 + 0.4637672555408563478e-4 * t34367 - 0.9275345110817126956e-4 * t34370 + 0.13672076938352463841e-4 * t34373;
    (t38082, t38086, t38088, t38093, t38118)
}
