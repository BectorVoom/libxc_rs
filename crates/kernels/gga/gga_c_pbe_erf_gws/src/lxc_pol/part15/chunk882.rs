//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 882/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk882<F: Float>(t617: F, t631: F, t184: F, t1024: F, t2724: F, t633: F, t5355: F, t1648: F, t2632: F, t2784: F, t597: F, t562: F) -> (F, F, F, F, F) {
    let t7631 = t617 * t631;
    let t7632 = t7631 * t184;
    let t7634 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t7632 * t1024;
    let t7636 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t633 * t2724;
    let t7637 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t5355;
    let t7639 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1648 * t2632;
    let t7640 = t597 * t2784;
    let t7641 = t7640 * t562;
    (t7634, t7636, t7637, t7639, t7641)
}
