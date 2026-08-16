//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 954/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk954<F: Float>(t1987: F, t9090: F, t1990: F, t1173: F, t674: F, t9085: F, t1997: F, t7696: F, t8676: F, t1986: F, t5251: F, t675: F) -> (F, F, F, F, F) {
    let t40354 = t9090 * t1987;
    let t40356 = t9090 * t1990;
    let t40357 = F::cast_from(0.19863479950205658386e-4_f64) * t40356;
    let t40359 = t9085 * t1173 * t674;
    let t40360 = t40359 * t1997;
    let t40362 = t8676 * t7696;
    let t40365 = t675 * t1986 * t5251;
    (t40354, t40357, t40360, t40362, t40365)
}
