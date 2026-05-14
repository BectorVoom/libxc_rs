//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 679/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk679<F: Float>(t201: F, t4443: F, t1976: F, t674: F, t2185: F, t7472: F, t16155: F, t7229: F) -> (F, F, F, F) {
    let t34855 = t201 * t4443;
    let t34857 = t1976 * t34855 * t674;
    let t34881 = t7472 * t2185;
    let t34884 = t7229 * t16155;
    (t34855, t34857, t34881, t34884)
}
