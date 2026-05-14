//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 847/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk847<F: Float>(t1579: F, t7398: F, t7071: F, t72: F, t8006: F, t686: F, t25375: F, t25387: F, t27240: F, t25246: F, t25257: F, t25267: F, t26450: F, t26454: F, t27222: F, t27224: F, t27226: F, t27228: F, t27230: F, t27232: F, t27234: F, t27236: F) -> (F, F, F, F) {
    let t28309 = t7398 * t1579;
    let t28310 = t7071 * t28309;
    let t28313 = t8006 * t72;
    let t28314 = t28313 * t686;
    let t28315 = t25375 * t28314;
    let t28317 = t25387 * t28314;
    let t28330 = 0.11433071498151929859e-3 * t27240;
    let t28331 = -0.50820002809285328225e-4 * t25246 + 0.40015750243531754507e-2 * t25267 + t27222 / 8.0 + 0.17149607247227894789e-1 * t27224 - 0.85748036236139473944e-3 * t27226 - 0.50820002809285328225e-4 * t27228 + 0.40015750243531754507e-2 * t27230 + 0.34299214494455789578e-2 * t27232 - 0.85748036236139473944e-3 * t27234 + 0.34299214494455789578e-2 * t27236 + t26450 - t26454 + t25257 + t28330;
    (t28310, t28315, t28317, t28331)
}
