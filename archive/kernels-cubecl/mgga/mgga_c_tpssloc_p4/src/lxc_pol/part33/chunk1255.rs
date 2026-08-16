//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1255/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1255<F: Float>(t1649: F, t2752: F, t1410: F, t9239: F, t26083: F, t45844: F, t6489: F, t22716: F, t7697: F, t7692: F, t81186: F, t1834: F, t794: F) -> (F, F, F, F, F, F, F) {
    let t89992 = t2752 * t1649;
    let t90137 = t9239 * t1410;
    let t90192 = t9239 * t26083;
    let t90330 = t45844 * t6489;
    let t90503 = t22716 * t7697;
    let t90521 = t81186 * t7692;
    let t90544 = t794 * t1834;
    (t89992, t90137, t90192, t90330, t90503, t90521, t90544)
}
