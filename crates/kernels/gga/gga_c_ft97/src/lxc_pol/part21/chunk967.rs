//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 967/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk967<F: Float>(t1017: F, t1039: F, t2185: F, t5900: F, t23657: F, t23671: F, t6656: F, t925: F, t4822: F, t5916: F, t23667: F, t5899: F, t23892: F, t4417: F, t1969: F, t446: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30211 = t1039 * t1017;
    let t30213 = t2185 * t5900 * t30211;
    let t30214 = t23657 * t30213;
    let t30220 = t23671 * t6656 * t925;
    let t30221 = t23657 * t30220;
    let t30223 = t5916 * t4822;
    let t30224 = t23667 * t30223;
    let t30225 = t5899 * t30224;
    let t30227 = t23892 * t4417;
    let t30228 = t1969 * t30227;
    let t30229 = t446 * t30228;
    (t30211, t30213, t30214, t30220, t30221, t30223, t30224, t30225, t30228, t30229)
}
