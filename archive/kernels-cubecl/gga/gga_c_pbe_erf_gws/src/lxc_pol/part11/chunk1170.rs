//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1170/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1170<F: Float>(t48439: F, t48476: F, t48487: F, t48490: F, t48494: F, t48500: F, t48509: F, t48517: F, t153: F, t156: F, t18359: F, t18363: F, t18372: F, t18413: F, t18415: F, t18419: F, t22778: F, t22800: F, t242: F, t25569: F, t34371: F, t43153: F, t48436: F) -> (F, F) {
    let t48520 = t48439 + t48476 + t48487 + t48490 + t48494 + t48500 + t48509 + t48517;
    let t48528 = F::cast_from(0.77820516338105134659e0_f64) * t22778 - F::cast_from(0.83762820535504401876e-1_f64) * t48436 * t242 + F::cast_from(0.42708890021612718669e0_f64) * t153 * t156 * t48520 + t18359 + F::cast_from(0.79723261373677074846e1_f64) * t34371 - t18363 + t18372 - t18413 + t18415 + F::cast_from(0.33505128214201760751e0_f64) * t43153 - t18419 - F::cast_from(0.2010307692852105645e1_f64) * t22800 + F::cast_from(0.2010307692852105645e1_f64) * t25569;
    (t48520, t48528)
}
