//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 295/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk295<F: Float>(t1121: F, t359: F, t376: F, t1170: F, t41: F, t982: F) -> (F, F, F, F) {
    let t1171 = t359 * t1121;
    let t1172 = t376 * t1171;
    let t1173 = t1170 * t1172;
    let t1175 = t982 * t41;
    (t1171, t1172, t1173, t1175)
}
