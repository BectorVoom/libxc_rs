//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 848/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk848<F: Float>(t275: F, t9031: F, t118: F, t2281: F, t498: F, t7418: F, t7244: F, t9153: F, t8876: F, t942: F, t4961: F, t668: F) -> (F, F, F, F, F) {
    let t41905 = F::new(2.0) * t275 * t9031;
    let t41914 = t7418 * t118 * t2281 * t498;
    let t41922 = t7244 * t9153;
    let t41929 = F::new(0.4726e1) * t942 * t8876;
    let t41932 = t4961 * t668;
    (t41905, t41914, t41922, t41929, t41932)
}
