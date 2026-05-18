//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 423/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk423<F: Float>(t2360: F, t241: F, t2349: F, t666: F, t89: F, t1934: F, t669: F, t240: F, t674: F) -> (F, F, F, F, F) {
    let t2361 = t241 * t2360;
    let t2362 = t2361 * t2349;
    let t2364 = t89 * t666 * t2362;
    let t2366 = t669 * t1934;
    let t2368 = t89 * t666 * t2366;
    let t2370 = t674 * t240;
    let t2371 = F::new(1.0) / t2370;
    (t2362, t2364, t2366, t2368, t2371)
}
