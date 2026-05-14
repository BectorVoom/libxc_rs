//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1317/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1317<F: Float>(t105444: F, t23671: F, t5899: F, t3188: F, t95379: F, t27072: F, t2120: F, t23657: F, t5916: F, t920: F, t23649: F, t27065: F, t27069: F, t105427: F, t105431: F, t105434: F, t105438: F, t105442: F, t96057: F, t96060: F) -> (F, F, F, F, F, F, F) {
    let t105446 = t5899 * t23671 * t105444;
    let t105448 = t95379 * t3188;
    let t105450 = t5899 * t27072 * t105448;
    let t105455 = t23657 * t23671 * t5916 * t920 * t2120;
    let t105457 = t23649 * t27065;
    let t105458 = 2.0 / 9.0 * t105457;
    let t105459 = t23649 * t27069;
    let t105460 = 2.0 / 9.0 * t105459;
    let t105461 = 8.0 / 3.0 * t105427 - 8.0 / 9.0 * t105431 + t105434 + 3.0 / 2.0 * t105438 - t96057 - t96060 + 2.0 / 3.0 * t105442 - 2.0 / 3.0 * t105446 + 2.0 / 9.0 * t105450 - t105455 / 12.0 + t105458 + t105460;
    (t105446, t105448, t105450, t105455, t105457, t105459, t105461)
}
