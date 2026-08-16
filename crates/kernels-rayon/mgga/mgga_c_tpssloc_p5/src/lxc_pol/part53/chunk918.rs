//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 918/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk918(t2053: f64, t2718: f64, t7841: f64, t1492: f64, t8728: f64, t31976: f64, t31978: f64, t31982: f64, t32835: f64, t32838: f64, t32841: f64, t32845: f64, t32847: f64) -> (f64, f64, f64) {
    let t33935 = t2718 * t2053 * t7841;
    let t33940 = t1492 * t8728;
    let t33947 = -t31976 - 0.19378922925187387609e-1_f64 * t32835 - t31978 - 0.32298204875312312682e-2_f64 * t32838 + t32841 / 384.0_f64 - t32845 / 384.0_f64 - t31982 - t32847 / 96.0_f64;
    (t33935, t33940, t33947)
}
