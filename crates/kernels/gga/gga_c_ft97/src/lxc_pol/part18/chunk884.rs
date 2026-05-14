//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 884/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk884<F: Float>(t23631: F, t586: F, t28: F, t5890: F, t1359: F, t2120: F, t1651: F, t1969: F, t5900: F, t5899: F, t1643: F, t9049: F, t1368: F, t458: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23632 = t586 * t23631;
    let t23634 = t5890 * t28 * t23632;
    let t23636 = t1359 * t2120;
    let t23637 = t586 * t23636;
    let t23639 = t5890 * t28 * t23637;
    let t23642 = t1969 * t5900 * t1651;
    let t23643 = t5899 * t23642;
    let t23646 = t9049 * t5900 * t1643;
    let t23647 = t5899 * t23646;
    let t23649 = t1368 * t458;
    (t23632, t23634, t23637, t23639, t23642, t23643, t23646, t23647, t23649)
}
