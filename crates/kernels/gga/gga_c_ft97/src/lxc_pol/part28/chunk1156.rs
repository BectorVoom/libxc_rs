//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1156/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1156<F: Float>(t148417: F, t39693: F, t446: F, t32063: F, t32888: F, t34809: F, t34918: F, t558: F, t1369: F, t2112: F, t28: F, t139507: F, t139519: F, t139526: F, t139534: F, t148640: F, t148643: F, t148646: F, t148649: F, t148653: F, t148657: F, t148660: F, t148667: F, t148670: F) -> (F, F, F, F, F) {
    let t148673 = t446 * t39693 * t148417;
    let t148676 = t32888 * t32063 * t34809;
    let t148678 = t34918 * t558;
    let t148681 = t1369 * t28 * t2112 * t148678;
    let t148683 = t148640 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t148643 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t148646 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t148649 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t148653 - F::cast_from(2.0_f64) * t148657 + t148660 / F::cast_from(18.0_f64) - t139507 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t139519 + t139526 / F::cast_from(18.0_f64) - t139534 + t148667 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t148670 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t148673 - t148676 / F::cast_from(3.0_f64) + t148681 / F::cast_from(3.0_f64);
    (t148673, t148676, t148678, t148681, t148683)
}
