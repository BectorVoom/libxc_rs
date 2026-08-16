//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 856/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk856<F: Float>(t242: F, t35729: F, t1175: F, t729: F, t7484: F, t1449: F, t6940: F, t2568: F, t35694: F, t35699: F, t35703: F, t35707: F, t35710: F, t35714: F, t35717: F, t35721: F, t35726: F, t446: F) -> (F, F, F, F, F, F) {
    let t35730 = t242 * t35729;
    let t35734 = t729 * t1175 * t7484;
    let t35737 = t1449 * t6940;
    let t35738 = t2568 * t35737;
    let t35739 = t242 * t35738;
    let t35742 = -t446 * t35694 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t35699 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t35703 - t446 * t35707 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t35710 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t35714 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t35717 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t35721 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t35726 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t35730 - t446 * t35734 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t35739;
    (t35730, t35734, t35737, t35738, t35739, t35742)
}
