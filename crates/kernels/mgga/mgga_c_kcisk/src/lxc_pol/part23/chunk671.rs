//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 671/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk671<F: Float>(t139: F, t41: F, t5911: F, t1422: F, t1423: F, t220: F, t1417: F, t2222: F, t1421: F, t3519: F, t3522: F, t3524: F, t3526: F, t5893: F, t5896: F, t5900: F, t5904: F, t5908: F) -> (F, F, F) {
    let t5913 = t139 * t5911 * t41;
    let t5915 = t1422 * t1423 * t220;
    let t5918 = t1417 * t2222;
    let t5920 = -t3519 + 0.43802864444444444445e-3 * t3522 + 0.98556445e-3 * t3524 - 0.65704296666666666667e-3 * t3526 + 0.43802864444444444445e-3 * t5893 + 0.10950716111111111111e-2 * t1421 * t5896 + 0.98556445e-3 * t1421 * t5900 - 0.65704296666666666667e-3 * t1421 * t5904 - 0.13140859333333333333e-2 * t1421 * t5908 + 0.13140859333333333333e-2 * t5913 * t5915 + 0.98556445e-3 * t5918;
    (t5913, t5915, t5920)
}
