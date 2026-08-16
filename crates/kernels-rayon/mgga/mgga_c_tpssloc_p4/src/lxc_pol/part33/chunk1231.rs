//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1231/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1231(t81849: f64, t1887: f64, t206: f64, t80845: f64, t23102: f64, t80782: f64, t23093: f64, t281: f64, t23046: f64, t812: f64, t835: f64, t22813: f64, t6589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81850 = 0.10173934535723378495e0_f64 * t81849;
    let t81852 = t80845 * t206 * t1887;
    let t81853 = 455.0_f64 / 1296.0_f64 * t81852;
    let t81876 = t23102 * t80782;
    let t81882 = t23093 * t281;
    let t81886 = t812 * t23046 * t835;
    let t81902 = t22813 * t6589 * t80782;
    (t81850, t81853, t81876, t81882, t81886, t81902)
}
