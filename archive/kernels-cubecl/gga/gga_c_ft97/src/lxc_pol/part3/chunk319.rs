//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 319/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk319<F: Float>(t35: F, t63: F, t78: F, t37: F, t62: F, t371: F, t66: F) -> (F, F, F, F, F, F) {
    let t1597 = t35 * t35;
    let t1598 = t1597 * t63;
    let t1599 = t1598 * t78;
    let t1602 = t37 * t62;
    let t1603 = t1602 * t78;
    let t1608 = t371 * t66;
    let t1609 = F::powf(F::cast_from(4.0_f64), F::cast_from(1.0_f64) / F::cast_from(15.0_f64));
    (t1597, t1599, t1602, t1603, t1608, t1609)
}
