//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 262/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk262<F: Float>(t472: F, t998: F, t1201: F, t1206: F, t206: F, t207: F, t470: F, t473: F) -> (F, F) {
    let t1209 = t472 * t998;
    let t1212 = -t1201 * t207 - F::new(12.0) * t1206 * t206 + F::new(3.0) * t1209 * t206 + F::new(6.0) * t470 * t473;
    (t1209, t1212)
}
