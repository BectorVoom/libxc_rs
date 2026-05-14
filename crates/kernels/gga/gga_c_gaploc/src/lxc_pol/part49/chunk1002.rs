//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1002/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1002<F: Float>(t12065: F, t2437: F, t41853: F, t41854: F, t41863: F, t41867: F, t41871: F, t41874: F, t41876: F, t41880: F, t47925: F, t47926: F, t47927: F, t2441: F, t38759: F, t895: F) -> (F, F, F) {
    let t47934 = t2437 * t12065;
    let t47936 = t41853 - t41854 - t47925 + t47926 - 0.10725146985555128001e1 * t47927 + 0.11502877786176224903e2 * t41863 + 0.11502877786176224903e2 * t41867 + 0.11502877786176224903e2 * t41871 + t41874 + 0.69017266717057349418e1 * t41876 - 0.21450293971110256001e1 * t41880 + 0.35750489951850426669e0 * t47934;
    let t47937 = t2441 * t12065;
    let t47939 = t895 * t38759;
    (t47936, t47937, t47939)
}
