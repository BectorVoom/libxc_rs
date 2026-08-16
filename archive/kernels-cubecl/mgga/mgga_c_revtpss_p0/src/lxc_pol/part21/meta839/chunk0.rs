//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3146/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3146<F: Float>(t12469: F, t1737: F, t43762: F, t43771: F, t43773: F, t43781: F, t43783: F, t43785: F, t43787: F, t45106: F, t45107: F, t56151: F, t56155: F) -> (F, F) {
    let t58005 = t1737 * t12469;
    let t58023 = -F::cast_from(0.30872592592592592592e-1_f64) * t43762 - F::cast_from(0.92617777777777777776e0_f64) * t43771 + F::cast_from(0.13892666666666666667e0_f64) * t43773 + F::cast_from(0.34731666666666666666e0_f64) * t43781 + F::cast_from(0.69463333333333333333e0_f64) * t43783 - F::cast_from(0.69463333333333333333e-1_f64) * t43785 - F::cast_from(0.41678000000000000001e0_f64) * t43787 + t45106 + t45107 - F::cast_from(0.123954e2_f64) * t56151 + F::cast_from(0.309885e1_f64) * t56155;
    (t58005, t58023)
}
