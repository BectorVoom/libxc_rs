//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1061/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1061<F: Float>(t108078: F, t108081: F, t108084: F, t108086: F, t108091: F, t108095: F, t108099: F, t108104: F, t108107: F, t108112: F, t108115: F, t108118: F, t1424: F, t9895: F, t13702: F, t1901: F) -> (F, F) {
    let t108119 = -t108078 - t108081 - t108084 + 4.0 / 3.0 * t108086 + t108091 + 2.0 * t108095 + 2.0 * t108099 + t108104 - t108107 / 3.0 + 2.0 / 3.0 * t108112 - t108115 - t108118;
    let t108120 = t9895 * t1424;
    let t108122 = t1901 * t108120 * t13702;
    (t108119, t108122)
}
