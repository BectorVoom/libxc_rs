//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1437/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1437(t1890: f64, t1966: f64, t28813: f64, t28816: f64, t28820: f64, t28822: f64, t33690: f64, t33692: f64, t33695: f64, t33702: f64, t33705: f64, t33708: f64, t33711: f64, t33713: f64, t33716: f64, t33722: f64, t33728: f64, t38907: f64, t590: f64) -> f64 {
    let t39268 = -t33690 - 0.51123901271894332902e1_f64 * t1966 * t1890 * t38907 * t590 + t33692 - t33695 + t33702 + t33705 - t33708 + t33711 + t33713 + t33716 - t33722 + t28813 - t28816 + t33728 + 0.76685851907841499354e0_f64 * t28820 - 0.10224780254378866581e1_f64 * t28822;
    t39268
}
