//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 847/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk847<F: Float>(t34506: F, t34507: F, t41726: F, t12766: F, t1572: F, t4673: F, t12919: F, t4953: F, t1445: F, t1562: F, t3116: F, t8097: F) -> (F, F, F, F) {
    let t42005 = F::cast_from(0.85801175884441024004e1_f64) * t34506 * t34507 * t41726;
    let t42008 = F::cast_from(0.47667319935800568892e0_f64) * t1572 * t4673 * t12766;
    let t42018 = F::cast_from(0.69017266717057349418e1_f64) * t4953 * t12919;
    let t42022 = F::cast_from(0.69017266717057349418e1_f64) * t1562 * t1445 * t8097 * t3116;
    (t42005, t42008, t42018, t42022)
}
