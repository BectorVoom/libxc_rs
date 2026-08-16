//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1433/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1433(t2355: f64, t8435: f64, t27229: f64, t7826: f64, t1961: f64, t33986: f64, t33988: f64, t33991: f64, t33992: f64, t33997: f64, t34003: f64, t34006: f64, t34008: f64, t34010: f64, t34012: f64, t34013: f64, t34018: f64, t34020: f64, t34023: f64, t3511: f64, t35239: f64, t5559: f64, t841: f64) -> (f64, f64) {
    let t35240 = t2355 * t8435;
    let t35242 = 6.0_f64 * t27229 * t7826;
    let t35243 = -6.0_f64 * t1961 * t3511 * t5559 + 2.0_f64 * t1961 * t33992 - 2.0_f64 * t34013 * t841 - t33986 + t33988 + t33991 - t33997 + t34003 + t34006 - t34008 + t34010 - t34012 + t34018 - t34020 - t34023 + t35239 + t35240 - t35242;
    (t35240, t35243)
}
