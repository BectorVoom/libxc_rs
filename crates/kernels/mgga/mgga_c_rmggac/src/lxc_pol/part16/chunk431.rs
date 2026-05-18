//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 431/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk431<F: Float>(t1134: F, t1138: F, t418: F, t971: F, t977: F, t431: F, t1038: F, t416: F, t4189: F, t1028: F, t385: F, t381: F) -> (F, F, F, F, F) {
    let t4352 = t1134 * t1138;
    let t4359 = t977 * t971 * t418;
    let t4361 = F::new(0.35089341735807877242e1) * t431 * t4359;
    let t4363 = t1038 * t416 * t4189;
    let t4365 = F::new(0.51947577317044391277e2) * t431 * t4363;
    let t4366 = t385 * t1028;
    let t4372 = t381 * t1028;
    (t4352, t4361, t4365, t4366, t4372)
}
