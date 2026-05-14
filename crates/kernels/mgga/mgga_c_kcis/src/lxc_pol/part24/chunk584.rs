//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 584/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk584<F: Float>(t2894: F, t6521: F, t2899: F, t6272: F, t993: F, t6276: F, t994: F, t1704: F) -> (F, F, F, F, F, F) {
    let t6522 = t2894 * t6521;
    let t6525 = t2899 * t6272;
    let t6526 = t993 * t6525;
    let t6529 = t994 * t6276;
    let t6530 = t993 * t6529;
    let t6533 = t1704 * t1704;
    (t6522, t6525, t6526, t6529, t6530, t6533)
}
