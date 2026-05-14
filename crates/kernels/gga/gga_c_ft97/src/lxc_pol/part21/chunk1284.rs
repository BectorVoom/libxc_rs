//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1284/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1284<F: Float>(t119959: F, t105909: F, t105923: F, t119834: F, t27081: F, t105678: F, t105686: F, t105697: F, t105711: F, t119936: F, t119939: F, t119943: F, t119948: F, t119953: F, t119956: F, t119735: F, t23671: F, t5899: F) -> (F, F, F) {
    let t119960 = 2.0 / 3.0 * t119959;
    let t119963 = t105923 * t105909 * t119834 * t27081;
    let t119965 = t119936 + t119939 - t105678 + t105686 - 3.0 / 4.0 * t119943 - 3.0 / 8.0 * t119948 + 15.0 / 16.0 * t119953 + t119956 - t105697 + 2.0 / 9.0 * t105711 - t119960 + t119963 / 4.0;
    let t119968 = t5899 * t23671 * t119735;
    (t119963, t119965, t119968)
}
