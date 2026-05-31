//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 948/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk948<F: Float>(t33571: F, t5996: F, t1774: F, t6144: F, t7426: F, t1439: F, t8281: F, t1526: F, t6136: F, t9483: F, t33540: F, t33543: F) -> (F, F, F, F, F) {
    let t141435 = t5996 * t33571;
    let t141441 = t7426 * t1774 * t6144;
    let t141447 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t7426 * t8281 * t1439;
    let t141461 = t1526 * t9483 * t6136;
    let t141468 = t33540 * t33543;
    (t141435, t141441, t141447, t141461, t141468)
}
