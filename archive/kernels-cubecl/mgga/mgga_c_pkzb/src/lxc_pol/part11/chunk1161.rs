//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1161/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1161<F: Float>(t2023: F, t3857: F, t3214: F, t8315: F, t3882: F, t5939: F, t918: F, t10197: F, t2376: F, t179: F, t19155: F, t3757: F, t404: F) -> (F, F, F, F, F) {
    let t28287 = t3857 * t2023;
    let t28295 = t3214 * t8315;
    let t28303 = t918 * t5939 * t3882;
    let t28305 = t10197 * t2376;
    let t28316 = t404 * t179 * t19155 * t3757;
    (t28287, t28295, t28303, t28305, t28316)
}
