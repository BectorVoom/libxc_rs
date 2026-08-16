//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1168/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1168(t10205: f64, t64: f64, t3334: f64, t90: f64, t29896: f64, t29898: f64, t29901: f64, t29911: f64, t29913: f64, t29915: f64, t10257: f64, t3833: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31612 = 8.0_f64 / 3.0_f64 * t10205 * t64;
    let t31614 = 4.0_f64 / 3.0_f64 * t3334 * t90;
    let t31617 = 63.0_f64 / 512.0_f64 * t29896;
    let t31618 = 385.0_f64 / 16384.0_f64 * t29898;
    let t31619 = 147.0_f64 / 1048576.0_f64 * t29901;
    let t31620 = 49.0_f64 / 1048576.0_f64 * t29911;
    let t31621 = 385.0_f64 / 49152.0_f64 * t29913;
    let t31622 = 21.0_f64 / 512.0_f64 * t29915;
    let t31646 = 0.1138200265427045984e0_f64 * t3833 * t10257;
    (t31612, t31614, t31617, t31618, t31619, t31620, t31621, t31622, t31646)
}
