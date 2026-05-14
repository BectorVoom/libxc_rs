//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 604/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk604<F: Float>(t2665: F, t28520: F, t446: F, t25037: F, t3886: F, t10409: F, t1486: F, t681: F, t7075: F, t1882: F, t7080: F, t668: F, t7021: F, t505: F, t28491: F, t28494: F, t28499: F, t28504: F, t28509: F, t28514: F, t28518: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28521 = t2665 * t28520;
    let t28522 = t446 * t28521;
    let t28524 = t25037 * t3886;
    let t28525 = t10409 * t28524;
    let t28526 = t446 * t28525;
    let t28529 = t1486 * t681 * t7075;
    let t28531 = t1882 * t7080;
    let t28533 = t7021 * t668;
    let t28534 = t28533 * t505;
    let t28535 = t2665 * t28534;
    let t28536 = t446 * t28535;
    let t28538 = t28491 / 9.0 - t28494 / 36.0 + t28499 / 3.0 + t28504 / 3.0 + t28509 / 3.0 + t28514 / 12.0 - 2.0 / 9.0 * t28518 - 2.0 / 9.0 * t28522 + 2.0 / 27.0 * t28526 + t28529 / 18.0 - t28531 / 27.0 + t28536 / 9.0;
    (t28521, t28522, t28524, t28525, t28526, t28529, t28531, t28534, t28536, t28538)
}
