//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 637/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk637(t4304: f64, t79: f64, t4208: f64, t469: f64, t41: f64, t470: f64, t3784: f64, t4229: f64, t499: f64, t260: f64, t338: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6322 = t79 * t4304;
    let t6331 = t4208 * t469;
    let t6332 = t41 * t470;
    let t6368 = t3784 * t4229;
    let t6369 = t79 * t499;
    let t6442 = t260 * t67 * t338;
    (t6322, t6331, t6332, t6368, t6369, t6442)
}
