//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 798/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk798<F: Float>(t34975: F, t34976: F, t39866: F, t7448: F, t16043: F, t8504: F, t2186: F, t8582: F, t2347: F, t833: F, t262: F, t8640: F, t848: F, t7198: F, t1165: F, t1979: F, t1982: F, t201: F, t589: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t39869 = t34975 * t34976 * t39866 * t7448;
    let t39871 = t16043 * t8504;
    let t39873 = t2186 * t8582;
    let t39875 = t2347 * t833;
    let t39876 = t262 * t39875;
    let t39877 = t8640 * t39876;
    let t39879 = t2347 * t848;
    let t39880 = t262 * t39879;
    let t39881 = t7198 * t39880;
    let t39889 = t589 * t1165 * t201 * t1979 * t1982;
    (t39869, t39871, t39873, t39875, t39876, t39877, t39879, t39880, t39881, t39889)
}
