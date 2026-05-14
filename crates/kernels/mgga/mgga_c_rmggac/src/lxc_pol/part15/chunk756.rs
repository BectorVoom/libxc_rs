//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 756/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk756<F: Float>(t2085: F, t8339: F, t1692: F, t2046: F, t2050: F, t31: F, t1635: F, t2064: F, t4044: F, t1550: F, t7778: F, t8377: F, t1632: F, t3928: F, t2373: F, t7561: F) -> (F, F, F, F, F, F) {
    let t41656 = t8339 * t2085;
    let t41657 = 0.18183107769496894486e-1 * t41656;
    let t41667 = t2046 * t2050 * t1692 * t31;
    let t41668 = 0.43368970657079495312e-4 * t41667;
    let t41716 = t4044 * t2064 * t1635;
    let t41717 = 0.95793933614910468512e0 * t41716;
    let t41722 = t1550 * t7778 * t8377;
    let t41723 = 0.15965655602485078085e0 * t41722;
    let t41725 = t3928 * t2064 * t1632;
    let t41726 = 0.47896966807455234256e0 * t41725;
    let t41727 = t2373 * t7561;
    (t41657, t41668, t41717, t41723, t41726, t41727)
}
