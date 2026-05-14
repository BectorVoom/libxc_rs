//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1109/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1109<F: Float>(t43548: F, t91: F, t1486: F, t2399: F, t6327: F, t2360: F, t6260: F, t1487: F, t9555: F, t1900: F, t2755: F, t6: F) -> (F, F, F, F, F, F) {
    let t99475 = t91 * t43548;
    let t99509 = t1486 * t2399 * t6327;
    let t99511 = t6260 * t2360;
    let t99524 = t1486 * t9555 * t1487;
    let t99525 = 14.0 / 27.0 * t99524;
    let t99528 = t91 * t2755 * t6 * t1900;
    (t99475, t99509, t99511, t99524, t99525, t99528)
}
