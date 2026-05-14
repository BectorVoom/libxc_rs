//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 917/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk917<F: Float>(t2868: F, t8997: F, t1971: F, t236: F, t6099: F, t8517: F, t10050: F, t34857: F, t1987: F, t47854: F, t1990: F, t1979: F, t1982: F, t458: F, t9774: F, t38530: F, t8422: F) -> (F, F, F, F, F, F, F) {
    let t47980 = t2868 * t8997;
    let t47984 = t8517 * t1971 * t236 * t6099;
    let t47986 = t34857 * t10050;
    let t47988 = t47854 * t1987;
    let t47990 = t47854 * t1990;
    let t47994 = t9774 * t458 * t1979 * t1982;
    let t47996 = t38530 * t8422;
    (t47980, t47984, t47986, t47988, t47990, t47994, t47996)
}
