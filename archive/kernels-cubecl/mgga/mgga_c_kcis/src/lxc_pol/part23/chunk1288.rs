//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1288/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1288<F: Float>(t27563: F, t28721: F, t2109: F, t27596: F, t4468: F, t6176: F, t4312: F, t94862: F, t98104: F, t1615: F, t6188: F, t7978: F, t99056: F) -> (F, F, F, F, F, F) {
    let t99069 = t28721 * t27563;
    let t99074 = t6176 * t27596 * t2109 * t4468;
    let t99079 = t6176 * t94862 * t2109 * t4312;
    let t99082 = F::cast_from(0.15476481481481481481e-2_f64) * t98104;
    let t99087 = t6176 * t27596 * t6188 * t1615;
    let t99098 = F::cast_from(0.23168402777777777778e-3_f64) * t7978 * t99056;
    (t99069, t99074, t99079, t99082, t99087, t99098)
}
