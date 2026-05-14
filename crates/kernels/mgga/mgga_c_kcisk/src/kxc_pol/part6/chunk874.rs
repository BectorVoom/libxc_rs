//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 874/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk874<F: Float>(t30273: F, t6183: F, t2168: F, t7744: F, t3937: F, t1224: F, t13538: F, t30233: F, t12951: F, t30153: F) -> (F, F, F, F) {
    let t30274 = t6183 * t30273;
    let t30277 = t7744 * t2168;
    let t30278 = t3937 * t30277;
    let t30288 = t1224 * t13538 * t30233;
    let t30290 = t12951 * t30153;
    (t30274, t30278, t30288, t30290)
}
