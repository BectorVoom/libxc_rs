//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 993/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk993<F: Float>(t2615: F, t326: F, t43586: F, t13146: F, t5676: F, t13077: F, t7712: F, t3040: F, t41468: F, t2536: F, t3431: F, t2009: F, t2021: F) -> (F, F, F, F, F) {
    let t43815 = t2615 * t326 * t43586;
    let t43817 = t5676 * t13146;
    let t43820 = F::cast_from(0.71500979903700853338e0_f64) * t13077 * t7712;
    let t43822 = F::cast_from(0.35750489951850426669e0_f64) * t41468 * t3040;
    let t43823 = t2536 * t3431;
    let t43825 = t2021 * t43823 * t2009;
    (t43815, t43817, t43820, t43822, t43825)
}
