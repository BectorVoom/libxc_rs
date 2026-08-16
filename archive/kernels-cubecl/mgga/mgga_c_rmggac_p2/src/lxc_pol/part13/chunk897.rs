//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 897/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk897<F: Float>(t511: F, t5752: F, t650: F, t338: F, t615: F, t34975: F, t34976: F, t7448: F, t16043: F, t8504: F, t2186: F, t8582: F) -> (F, F, F, F, F) {
    let t39863 = t5752 * t511;
    let t39864 = t39863 * t650;
    let t39866 = t338 * t615;
    let t39869 = t34975 * t34976 * t39866 * t7448;
    let t39871 = t16043 * t8504;
    let t39873 = t2186 * t8582;
    (t39864, t39866, t39869, t39871, t39873)
}
