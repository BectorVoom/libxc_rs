//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1103/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1103<F: Float>(t18331: F, t2970: F, t2177: F, t91: F, t204: F, t3981: F, t824: F) -> (F, F, F) {
    let t18332 = t2970 * t18331;
    let t18406 = t2177 * t2177;
    let t18408 = F::cast_from(1.0_f64) / t91 / t18406;
    let t18427 = t204 * t3981 * t824;
    (t18332, t18408, t18427)
}
