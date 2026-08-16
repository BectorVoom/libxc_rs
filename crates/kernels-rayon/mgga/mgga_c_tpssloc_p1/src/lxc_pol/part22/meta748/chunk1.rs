//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2501/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2501(t67059: f64, t18255: f64, t51667: f64, t18259: f64, t50819: f64, t22408: f64, t3640: f64, t1164: f64, t15218: f64, t18279: f64, t18910: f64, t18274: f64, t51651: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t71090 = -t67059;
    let t71095 = 18.0_f64 * t51667 * t18255;
    let t71097 = 0.2894756309764656312e3_f64 * t50819 * t18259;
    let t71101 = t22408 * t3640;
    let t71106 = 0.31168546390226634765e3_f64 * t1164 * t18279 * t15218;
    let t71109 = 0.51947577317044391277e2_f64 * t1164 * t18910 * t15218;
    let t71112 = 0.30762056574649219974e4_f64 * t1164 * t18274 * t51651;
    (t71090, t71095, t71097, t71101, t71106, t71109, t71112)
}
