//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 758/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk758<F: Float>(t7896: F, t8085: F, t157: F, t2152: F, t633: F, t929: F, t2176: F, t880: F, t639: F, t7924: F, t2217: F, t309: F) -> (F, F, F, F, F) {
    let t8087 = F::new(0.34694512752820797848e1) * t7896 * t8085;
    let t8092 = t2152 * t633 * t929 * t157;
    let t8096 = F::new(0.65854491829355115987e0) * t2176 * t880;
    let t8098 = F::new(0.8673628188205199462e0) * t7924 * t639;
    let t8099 = t2217 * t309;
    (t8087, t8092, t8096, t8098, t8099)
}
