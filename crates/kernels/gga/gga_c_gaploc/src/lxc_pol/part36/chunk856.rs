//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 856/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk856<F: Float>(t10418: F, t2389: F, t34506: F, t34507: F, t41726: F, t12766: F, t1572: F, t4673: F, t41822: F, t475: F, t10340: F, t1445: F, t1562: F, t2293: F) -> (F, F, F, F, F) {
    let t42001 = t10418 * t2389;
    let t42005 = F::new(0.85801175884441024004e1) * t34506 * t34507 * t41726;
    let t42008 = F::new(0.47667319935800568892e0) * t1572 * t4673 * t12766;
    let t42009 = t41822 * t475;
    let t42015 = t1562 * t1445 * t10340 * t2293;
    (t42001, t42005, t42008, t42009, t42015)
}
