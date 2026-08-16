//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1170/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1170(t48439: f64, t48476: f64, t48487: f64, t48490: f64, t48494: f64, t48500: f64, t48509: f64, t48517: f64, t153: f64, t156: f64, t18359: f64, t18363: f64, t18372: f64, t18413: f64, t18415: f64, t18419: f64, t22778: f64, t22800: f64, t242: f64, t25569: f64, t34371: f64, t43153: f64, t48436: f64) -> (f64, f64) {
    let t48520 = t48439 + t48476 + t48487 + t48490 + t48494 + t48500 + t48509 + t48517;
    let t48528 = 0.77820516338105134659e0_f64 * t22778 - 0.83762820535504401876e-1_f64 * t48436 * t242 + 0.42708890021612718669e0_f64 * t153 * t156 * t48520 + t18359 + 0.79723261373677074846e1_f64 * t34371 - t18363 + t18372 - t18413 + t18415 + 0.33505128214201760751e0_f64 * t43153 - t18419 - 0.2010307692852105645e1_f64 * t22800 + 0.2010307692852105645e1_f64 * t25569;
    (t48520, t48528)
}
