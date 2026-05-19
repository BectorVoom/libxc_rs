//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1242/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1242<F: Float>(t21497: F, t32616: F, t1897: F, t29190: F, t2936: F, t10704: F, t1850: F, t10636: F, t5227: F, t1841: F, t3487: F, t7275: F, t734: F) -> (F, F, F, F, F) {
    let t32618 = F::cast_from(0.34180116578409885704e-2_f64) * t21497 * t32616;
    let t32621 = F::cast_from(0.46143157380853345702e-1_f64) * t1897 * t2936 * t29190;
    let t32622 = t1850 * t10704;
    let t32623 = F::cast_from(0.85450291446024714264e-3_f64) * t32622;
    let t32625 = F::cast_from(0.17090058289204942853e-2_f64) * t5227 * t10636;
    let t32629 = F::cast_from(0.17090058289204942853e-2_f64) * t1841 * t7275 * t3487 * t734;
    (t32618, t32621, t32623, t32625, t32629)
}
