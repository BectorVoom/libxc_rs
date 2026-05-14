//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 858/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk858<F: Float>(t2367: F, t4616: F, t876: F, t2402: F, t794: F, t2134: F, t27: F, t4895: F, t649: F, t6355: F, t7810: F, t2344: F, t35674: F, t866: F, t8800: F, t36391: F, t9222: F) -> (F, F, F, F, F, F, F) {
    let t40596 = t4616 * t2367;
    let t40597 = t40596 * t876;
    let t40602 = t2402 * t794;
    let t40607 = t2134 * t27 * t649 * t4895;
    let t40610 = t6355 * t7810;
    let t40614 = t35674 * t2344;
    let t40616 = t8800 * t866;
    let t40619 = t9222 * t36391;
    (t40597, t40602, t40607, t40610, t40614, t40616, t40619)
}
