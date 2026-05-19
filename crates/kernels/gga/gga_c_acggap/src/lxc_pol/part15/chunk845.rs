//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 845/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk845<F: Float>(t8278: F, t8291: F, t8292: F, t8294: F, t8945: F, t8953: F, t8973: F, t8975: F, t8981: F, t8983: F, t9739: F, t9741: F, t9743: F, t9747: F, t9749: F, t9751: F, t9753: F, t9755: F, t9759: F, t9762: F) -> F {
    let t9968 = -t8278 - F::new(7.0) / F::new(72.0) * t8945 - F::cast_from(0.62896184579208304138e-3_f64) * t8953 - t9739 / F::new(12.0) - t9741 / F::new(24.0) + t9743 / F::new(8.0) + F::cast_from(0.12862205435420921092e-1_f64) * t8973 - F::cast_from(0.11321313224257494745e-1_f64) * t8975 - F::cast_from(0.18868855373762491241e-1_f64) * t8981 + F::cast_from(0.51448821741683684367e-2_f64) * t8983 - t9747 / F::new(24.0) - t9749 / F::new(48.0) + t9751 / F::new(24.0) + F::cast_from(0.17149607247227894789e-1_f64) * t9753 + F::cast_from(0.51448821741683684367e-2_f64) * t9755 + t8291 + t8292 - F::cast_from(0.21437009059034868486e-3_f64) * t9759 + t8294 + t9762 / F::new(48.0);
    t9968
}
