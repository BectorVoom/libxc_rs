//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 953/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk953<F: Float>(t24304: F, t23682: F, t2568: F, t212: F, t2263: F, t362: F, t508: F, t896: F, t297: F, t935: F, t22: F, t7856: F, t2672: F, t10: F, t2666: F, t2662: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t24305 = 1.0 / t24304;
    let t24321 = 0.96141975308641975307e-1 * t23682;
    let t24356 = t2568 * t2568;
    let t24357 = 1.0 / t24356;
    let t24391 = 1.0 / t212 / t2263;
    let t24392 = t24391 * t362;
    let t24407 = t508 * t896;
    let t24442 = t935 * t297;
    let t24447 = t22 * t7856;
    let t24468 = t2672 * t2263;
    let t24502 = t2666 * t10;
    let t24503 = t2662 * t24502;
    (t24305, t24321, t24357, t24391, t24392, t24407, t24442, t24447, t24468, t24502, t24503)
}
