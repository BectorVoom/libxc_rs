//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2035/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2035<F: Float>(t25904: F, t97899: F, t1358: F, t212: F, t27960: F, t689: F, t26050: F, t27899: F, t2453: F, t27883: F, t25946: F, t27873: F, t94890: F) -> (F, F, F, F, F) {
    let t97900 = t25904 * t97899;
    let t97908 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t212 * t27960 * t1358;
    let t97915 = F::cast_from(0.14456046980341999104e-1_f64) * t27899 * t26050;
    let t97916 = t2453 * t27883;
    let t97917 = t97916 * t25946;
    let t97920 = F::cast_from(0.28912093960683998208e-1_f64) * t94890 * t27873;
    (t97900, t97908, t97915, t97917, t97920)
}
