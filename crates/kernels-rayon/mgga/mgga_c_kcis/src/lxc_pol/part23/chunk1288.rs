//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1288/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1288(t27563: f64, t28721: f64, t2109: f64, t27596: f64, t4468: f64, t6176: f64, t4312: f64, t94862: f64, t98104: f64, t1615: f64, t6188: f64, t7978: f64, t99056: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99069 = t28721 * t27563;
    let t99074 = t6176 * t27596 * t2109 * t4468;
    let t99079 = t6176 * t94862 * t2109 * t4312;
    let t99082 = 0.15476481481481481481e-2_f64 * t98104;
    let t99087 = t6176 * t27596 * t6188 * t1615;
    let t99098 = 0.23168402777777777778e-3_f64 * t7978 * t99056;
    (t99069, t99074, t99079, t99082, t99087, t99098)
}
