//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 845/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk845<F: Float>(t15772: F, t167: F, t569: F, t2205: F, t4454: F, t616: F, t12963: F, t12965: F, t12967: F, t12975: F, t17101: F, t17104: F, t17107: F, t17111: F, t17115: F, t17120: F, t17125: F, t17129: F, t17133: F, t3281: F, t446: F) -> F {
    let t17137 = t569 * t167 * t15772;
    let t17141 = t2205 * t616 * t4454;
    let t17144 = F::new(2.0) / F::new(3.0) * t446 * t17101 - F::new(2.0) / F::new(27.0) * t17104 - F::new(2.0) / F::new(3.0) * t446 * t17107 - F::new(2.0) / F::new(9.0) * t446 * t17111 + F::new(2.0) / F::new(3.0) * t446 * t17115 + F::new(2.0) / F::new(3.0) * t446 * t17120 + F::new(2.0) / F::new(3.0) * t446 * t17125 - t12963 - t12965 - t12967 - t12975 - F::new(4.0) / F::new(9.0) * t3281 * t17129 - t446 * t17133 / F::new(9.0) - t446 * t17137 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t446 * t17141;
    t17144
}
