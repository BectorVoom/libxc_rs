//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 692/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk692<F: Float>(t1775: F, t3146: F, t3131: F, t1555: F, t26: F, t1557: F, t469: F, t356: F, t1570: F, t11069: F, t11076: F, t11416: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11734 = F::new(2.0) / F::new(9.0) * t1775 * t3146;
    let t11745 = F::new(2.0) / F::new(9.0) * t1775 * t3131;
    let t11755 = t26 * t1555;
    let t11756 = t469 * t1557;
    let t11761 = t26 * t356;
    let t11762 = t469 * t1570;
    let t11778 = F::new(2.0) / F::new(9.0) * t11069;
    let t11781 = F::new(4.0) / F::new(27.0) * t11076;
    let t11798 = F::new(4.0) / F::new(9.0) * t11416;
    (t11734, t11745, t11755, t11756, t11761, t11762, t11778, t11781, t11798)
}
