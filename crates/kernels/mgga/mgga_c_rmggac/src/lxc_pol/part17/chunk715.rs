//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 715/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk715<F: Float>(t10166: F, t262: F, t7835: F, t7844: F, t9885: F, t570: F, t8946: F, t4669: F, t9712: F, t7782: F, t9713: F, t7785: F, t9709: F) -> (F, F, F, F, F, F) {
    let t10168 = t7835 * t262 * t10166;
    let t10170 = t7844 * t9885;
    let t10174 = t8946 * t570;
    let t10177 = t4669 * t9712;
    let t10179 = t7782 * t9713;
    let t10181 = t7785 * t9709;
    (t10168, t10170, t10174, t10177, t10179, t10181)
}
