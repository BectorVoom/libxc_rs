//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1047/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1047<F: Float>(t107756: F, t107765: F, t107782: F, t107787: F, t107793: F, t107794: F, t107797: F, t13863: F, t13892: F, t14075: F, t14103: F, t14116: F, t24231: F, t24232: F, t24233: F, t2601: F, t28010: F, t28030: F, t28031: F, t28036: F, t28037: F, t6002: F) -> (F,) {
    let t107799 = t6002 * t28030 * t28031 * t14075 / 9.0 + 2.0 / 9.0 * t6002 * t28036 * t107756 * t13863 + t6002 * t24231 * t24232 * t14103 / 9.0 + 2.0 / 9.0 * t6002 * t24231 * t107765 * t2601 - 4.0 / 9.0 * t28010 * t24231 * t24232 * t13892 - 4.0 / 9.0 * t28010 * t28030 * t28031 * t14116 + 4.0 / 27.0 * t28010 * t28036 * t28037 * t14116 + 2.0 / 9.0 * t6002 * t107782 * t24233 - t107787 - t6002 * t28030 * t28037 * t13863 / 3.0 + t107793 + 4.0 * t107794 + 4.0 * t107797;
    (t107799,)
}
