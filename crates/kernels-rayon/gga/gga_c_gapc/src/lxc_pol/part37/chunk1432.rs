//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1432/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1432(t33680: f64, t33687: f64, t33692: f64, t36659: f64, t36660: f64, t36661: f64, t36662: f64, t36664: f64, t36666: f64, t36668: f64, t36669: f64, t33719: f64, t36671: f64, t36672: f64, t36673: f64, t36674: f64, t36675: f64, t36676: f64, t36678: f64, t36679: f64, t36680: f64, t36681: f64) -> (f64, f64) {
    let t38753 = -t36659 + t36660 - t36661 + t36662 - 0.2445773654513888889e-4_f64 * t33680 + t36664 - 0.2445773654513888889e-4_f64 * t33687 + t36666 + 0.56912804804009946682e-7_f64 * t33692 - t36668 + t36669;
    let t38755 = -t36671 - t36672 - t36673 - t36674 + t36675 - t36676 + 0.12650553385416666667e-5_f64 * t33719 + t36678 - t36679 - t36680 + t36681;
    (t38753, t38755)
}
