//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 406/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk406(t6035: f64, t6804: f64, t3766: f64, t6054: f64, t1113: f64, t231: f64, t39: f64, t694: f64, t5585: f64, t3789: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6805 = t6035 * t6804;
    let t6808 = t3766 * t6054;
    let t6809 = t231 * t1113;
    let t6813 = t694 * t39;
    let t6814 = t6813 * t5585;
    let t6815 = t3789 * t6814;
    (t6805, t6808, t6809, t6813, t6814, t6815)
}
