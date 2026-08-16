//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 897/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk897(t511: f64, t5752: f64, t650: f64, t338: f64, t615: f64, t34975: f64, t34976: f64, t7448: f64, t16043: f64, t8504: f64, t2186: f64, t8582: f64) -> (f64, f64, f64, f64, f64) {
    let t39863 = t5752 * t511;
    let t39864 = t39863 * t650;
    let t39866 = t338 * t615;
    let t39869 = t34975 * t34976 * t39866 * t7448;
    let t39871 = t16043 * t8504;
    let t39873 = t2186 * t8582;
    (t39864, t39866, t39869, t39871, t39873)
}
