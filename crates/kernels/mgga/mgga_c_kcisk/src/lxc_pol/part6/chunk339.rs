//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 339/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk339<F: Float>(t1162: F, t2077: F, t321: F, t1161: F) -> (F, F, F) {
    let t2079 = -t1162 - F::new(0.17808333333333333333e-1) * t2077;
    let t2081 = F::new(0.62182e-1) * t2079 * t321;
    let t2083 = -t1161 / F::new(3.0) - t2077 / F::new(3.0);
    (t2079, t2081, t2083)
}
