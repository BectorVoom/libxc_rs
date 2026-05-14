//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 758/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk758<F: Float>(t134: F, t2299: F, t941: F, t3405: F, t3403: F, t2639: F, t9832: F, t1069: F, t2795: F, t3375: F, t9903: F, t3367: F, t3374: F, t3371: F, t2405: F, t2636: F) -> (F, F, F, F, F, F, F, F) {
    let t9988 = t134 * t2299;
    let t9989 = t941 * t9988;
    let t9990 = t3405 * t9989;
    let t9991 = t3403 * t9990;
    let t9993 = t9832 * t2639;
    let t9995 = t1069 * t2795;
    let t9997 = t9903 * t3375;
    let t9999 = t3367 * t3374;
    let t10000 = t3371 * t9999;
    let t10002 = t2636 * t2405;
    (t9990, t9991, t9993, t9995, t9997, t9999, t10000, t10002)
}
