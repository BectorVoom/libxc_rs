//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 867/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk867<F: Float>(t23197: F, t23261: F, t23318: F, t23383: F, t1310: F, t1337: F, t1538: F, t1761: F, t22915: F, t22919: F, t22924: F, t22928: F, t22932: F, t22935: F, t22941: F, t22944: F, t22946: F, t22948: F, t22950: F, t23085: F, t23090: F, t23093: F, t23129: F, t23133: F, t438: F, t5501: F, t5504: F, t5748: F, t88: F) -> (F, F) {
    let t23385 = t23197 + t23261 + t23318 + t23383;
    let t23387 = t22915 / 27.0 - t5501 * t22919 / 9.0 - t5501 * t22924 / 9.0 - t5501 * t22928 / 18.0 - t5501 * t22932 / 27.0 - t22935 * t5504 / 9.0 - 2.0 * t438 * t5748 - 4.0 * t22941 + 4.0 * t22944 - 4.0 * t22946 - 2.0 * t22948 - 2.0 * t22950 - 2.0 * t23085 - t1538 * t1337 - t1761 * t1337 - t23090 / 9.0 - 12.0 * t23093 + 2.0 * t23129 + t23133 * t1310 / 6.0 - t88 * t23385;
    (t23385, t23387)
}
