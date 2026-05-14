//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 844/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk844<F: Float>(t551: F, t7817: F, t1550: F, t25441: F, t8410: F, t5016: F, t8542: F, t2289: F, t7939: F, t2323: F, t638: F, t7184: F, t2412: F, t7905: F, t1987: F, t9087: F) -> (F, F, F, F, F, F, F, F) {
    let t40331 = t7817 * t551;
    let t40332 = t1550 * t40331;
    let t40335 = t25441 * t8410;
    let t40337 = t5016 * t8542;
    let t40339 = t7939 * t2289;
    let t40343 = t638 * t7184 * t2323;
    let t40345 = t2412 * t7905;
    let t40347 = t9087 * t1987;
    (t40331, t40332, t40335, t40337, t40339, t40343, t40345, t40347)
}
