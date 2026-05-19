//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 634/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk634<F: Float>(t2704: F, t502: F, t1509: F, t2718: F, t486: F, t118: F, t119: F, t120: F, t837: F, t4516: F, t103: F, t2: F) -> (F, F, F, F, F, F) {
    let t5751 = F::cast_from(0.76172444444444444444e0_f64) * t502 * t2704;
    let t5753 = F::cast_from(0.12991222222222222222e1_f64) * t1509 * t2718;
    let t5755 = F::cast_from(0.15156425925925925926e1_f64) * t486 * t2704;
    let t5759 = F::new(7.0) / F::new(27.0) * t118 * t119 * t837 * t120;
    let t5772 = param_hyb_omega_0 * t4516;
    let t5773 = t103 * t2;
    (t5751, t5753, t5755, t5759, t5772, t5773)
}
