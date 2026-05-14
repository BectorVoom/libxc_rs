//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1276/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1276<F: Float>(t27165: F, t3450: F, t5899: F, t9432: F, t119768: F, t23667: F, t23649: F, t30175: F, t2185: F, t23652: F, t23657: F, t30211: F, t105511: F, t105517: F, t119797: F, t119800: F, t119803: F, t119807: F, t119810: F, t96064: F) -> (F, F, F, F, F) {
    let t119814 = t5899 * t9432 * t27165 * t3450;
    let t119817 = t5899 * t23667 * t119768;
    let t119819 = t23649 * t30175;
    let t119820 = t119819 / 9.0;
    let t119823 = t23657 * t2185 * t23652 * t30211;
    let t119824 = t96064 - t119797 + t119800 - 3.0 * t119803 - t119807 + t119810 / 3.0 - t105511 + t105517 - 6.0 * t119814 - 2.0 / 3.0 * t119817 - t119820 - t119823;
    (t119814, t119817, t119819, t119823, t119824)
}
