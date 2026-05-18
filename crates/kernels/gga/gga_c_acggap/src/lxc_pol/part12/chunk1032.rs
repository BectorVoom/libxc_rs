//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1032/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1032<F: Float>(t1083: F, t137: F, t34368: F, t4257: F, t1511: F, t2020: F, t31146: F, t4487: F, t7815: F, t2030: F, t5160: F, t7440: F, t8631: F) -> (F, F, F, F, F) {
    let t34369 = t1083 * t137;
    let t34371 = t34368 * t34369 * t4257;
    let t34382 = t2020 * t1511;
    let t34385 = t31146 * t7815 * t4487;
    let t34388 = t2030 * t7815 * t5160;
    let t34390 = t7440 * t8631;
    (t34371, t34382, t34385, t34388, t34390)
}
