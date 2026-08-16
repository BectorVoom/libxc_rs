//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1267/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1267<F: Float>(t81849: F, t1887: F, t206: F, t80845: F, t23102: F, t80782: F, t23093: F, t281: F, t23046: F, t812: F, t835: F, t22813: F, t6589: F) -> (F, F, F, F, F, F) {
    let t81850 = F::cast_from(0.10173934535723378495e0_f64) * t81849;
    let t81852 = t80845 * t206 * t1887;
    let t81853 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t81852;
    let t81876 = t23102 * t80782;
    let t81882 = t23093 * t281;
    let t81886 = t812 * t23046 * t835;
    let t81902 = t22813 * t6589 * t80782;
    (t81850, t81853, t81876, t81882, t81886, t81902)
}
