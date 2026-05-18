//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1047/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1047<F: Float>(t1511: F, t2020: F, t31146: F, t4487: F, t7815: F, t2030: F, t5160: F, t7440: F, t8631: F, t2318: F, t31261: F, t7538: F, t8689: F) -> (F, F, F, F, F, F) {
    let t34382 = t2020 * t1511;
    let t34383 = F::new(7.0) / F::new(144.0) * t34382;
    let t34385 = t31146 * t7815 * t4487;
    let t34388 = t2030 * t7815 * t5160;
    let t34390 = t7440 * t8631;
    let t34391 = F::new(0.5603125e-1) * t34390;
    let t34392 = t31261 * t2318;
    let t34394 = t7538 * t8689;
    (t34383, t34385, t34388, t34391, t34392, t34394)
}
