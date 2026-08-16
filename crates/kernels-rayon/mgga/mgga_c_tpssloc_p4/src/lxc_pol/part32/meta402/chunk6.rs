//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1533/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1533(t17766: f64, t17798: f64, t17852: f64, t17873: f64, t225: f64, t68: f64, t369: f64, t10457: f64, t248: f64, t5677: f64, t1041: f64, t1044: f64, t17187: f64) -> (f64, f64, f64, f64, f64) {
    let t17875 = t17766 + t17798 + t17852 + t17873;
    let t17876 = t17875 * t225;
    let t17877 = t17876 * t68;
    let t17878 = t17877 * t369;
    let t17884 = t248 * t10457 * t5677;
    let t17885 = t1041 * t17884;
    let t17890 = t248 * t1044 * t17187;
    (t17875, t17876, t17878, t17885, t17890)
}
