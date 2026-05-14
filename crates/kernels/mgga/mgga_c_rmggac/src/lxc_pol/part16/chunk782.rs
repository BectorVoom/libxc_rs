//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 782/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk782<F: Float>(t41667: F, t41716: F, t41722: F, t41725: F, t4961: F, t702: F, t2265: F, t5321: F, t41789: F, t41791: F, t41811: F, t41813: F, t1562: F, t8188: F, t41817: F, t41821: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43792 = 0.86737941314158990616e-4 * t41667;
    let t43810 = 0.19158786722982093702e1 * t41716;
    let t43812 = 0.3193131120497015617e0 * t41722;
    let t43813 = 0.95793933614910468512e0 * t41725;
    let t43817 = t4961 * t702;
    let t43836 = 0.4726e1 * t5321 * t2265;
    let t43844 = 0.3193131120497015617e0 * t41789;
    let t43850 = 0.3193131120497015617e0 * t41791;
    let t43861 = 0.39726959900411316772e-4 * t41811;
    let t43862 = 0.11918087970123395032e-3 * t41813;
    let t43864 = 0.4726e1 * t1562 * t8188;
    let t43868 = 0.1440846329149835838e-2 * t41817;
    let t43869 = 0.1440846329149835838e-2 * t41821;
    (t43792, t43810, t43812, t43813, t43817, t43836, t43844, t43850, t43861, t43862, t43864, t43868, t43869)
}
