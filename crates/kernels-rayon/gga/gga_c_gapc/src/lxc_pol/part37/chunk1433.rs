//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1433/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1433(t33779: f64, t36698: f64, t36699: f64, t36700: f64, t36701: f64, t36703: f64, t36704: f64, t36705: f64, t36706: f64, t36707: f64, t36708: f64, t33834: f64, t33838: f64, t36723: f64, t36725: f64, t36727: f64, t36728: f64, t36729: f64, t36730: f64, t36731: f64, t36732: f64, t36733: f64) -> (f64, f64) {
    let t38763 = -t36698 - t36699 - t36700 + t36701 - 0.57970906942607043475e-5_f64 * t33779 - t36703 + t36704 + t36705 - t36706 + t36707 + t36708;
    let t38767 = -t36723 + 0.2445773654513888889e-4_f64 * t33834 - t36725 - 0.18115908419564701086e-6_f64 * t33838 + t36727 - t36728 + t36729 - t36730 + t36731 + t36732 - t36733;
    (t38763, t38767)
}
