//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 554/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk554<F: Float>(t1977: F, t1982: F, t7428: F, t1165: F, t194: F, t201: F, t1979: F, t1987: F, t2186: F, t2034: F, t5016: F, t2061: F, t2604: F) -> (F, F, F, F, F, F, F) {
    let t7430 = t1977 * t7428 * t1982;
    let t7433 = t194 * t1165;
    let t7434 = t7433 * t201;
    let t7436 = t7434 * t1979 * t1982;
    let t7437 = F::new(0.42564599893297839398e-5) * t7436;
    let t7438 = t2186 * t1987;
    let t7440 = t5016 * t2034;
    let t7441 = F::new(0.5987120850931904282e-1) * t7440;
    let t7442 = t2604 * t2061;
    (t7430, t7433, t7434, t7437, t7438, t7441, t7442)
}
