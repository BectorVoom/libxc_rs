//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1440/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1440(t12161: f64, t12241: f64, t1890: f64, t1966: f64, t28792: f64, t28795: f64, t28800: f64, t28810: f64, t33659: f64, t33662: f64, t33666: f64, t33668: f64, t33671: f64, t33673: f64, t33675: f64, t33683: f64, t33685: f64, t5577: f64, t590: f64) -> f64 {
    let t39261 = t33659 - t33662 + t33666 + t33668 + t33671 - t33673 - t33675 + t33683 - t33685 + 0.10224780254378866581e1_f64 * t28792 + 0.10224780254378866581e1_f64 * t28795 + t28800 - 0.1022478025437886658e1_f64 * t5577 * t12241 - 0.1022478025437886658e1_f64 * t1966 * t1890 * t12161 * t590 - t28810;
    t39261
}
