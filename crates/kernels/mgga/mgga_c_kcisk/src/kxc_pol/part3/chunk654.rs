//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 654/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk654<F: Float>(t10442: F, t1801: F, t1800: F, t1799: F, t213: F, t220: F, t967: F) -> (F, F) {
    let t10443 = t1801 * t10442;
    let t10444 = t1800 * t10443;
    let t10445 = t1799 * t10444;
    let t10447 = t220 * t213;
    let t10449 = -F::new(6.0) * t967 + F::new(6.0) * t10447;
    (t10445, t10449)
}
