//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1102/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1102<F: Float>(t147: F, t87840: F, t88051: F, t21645: F, t3699: F, t4917: F, t5064: F, t1168: F, t21181: F, t1091: F, t14080: F, t1901: F, t2599: F, t42416: F, t51453: F, t65437: F, t65508: F, t79138: F, t79157: F, t79179: F, t79182: F, t79218: F, t80212: F, t81413: F) -> (F, F, F, F, F) {
    let t148 = F::new(10000000.0) <= t147;
    let t88053 = piecewise3::<f64>(t148, F::new(0.0), t87840 + t88051);
    let t88068 = t3699 * t21645;
    let t88079 = t4917 * t5064;
    let t88098 = t21181 * t1168;
    let t88103 = -F::new(16.0) / F::new(81.0) * t65437 + F::new(8.0) / F::new(9.0) * t79138 + F::new(8.0) / F::new(9.0) * t79157 + F::new(16.0) / F::new(9.0) * t65508 + F::new(112.0) / F::new(81.0) * t51453 + F::new(4.0) / F::new(3.0) * t79179 + F::new(4.0) / F::new(3.0) * t79182 + F::new(4.0) / F::new(3.0) * t79218 + F::new(4.0) / F::new(9.0) * t1901 * t2599 * t81413 * t1091 + F::new(4.0) / F::new(9.0) * t80212 + F::new(40.0) / F::new(81.0) * t1901 * t14080 * t42416 * t88098;
    (t88053, t88068, t88079, t88098, t88103)
}
