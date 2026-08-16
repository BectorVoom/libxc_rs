//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1004/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1004(t6188: f64, t6207: f64, t15896: f64, t1592: f64, t15941: f64, t17973: f64, t21027: f64, t21030: f64, t21033: f64, t21036: f64, t21041: f64, t21044: f64, t21048: f64, t21052: f64, t21055: f64, t21059: f64, t21508: f64, t21512: f64, t21514: f64, t22723: f64, t22759: f64, t4414: f64) -> (f64, f64) {
    let t22783 = t6207 * t6188;
    let t22792 = 0.12381185185185185185e-1_f64 * t21027 - 0.23214722222222222222e-2_f64 * t21030 - 0.34822083333333333332e-2_f64 * t21033 + 0.92858888888888888886e-2_f64 * t21036 - 0.38691203703703703703e-3_f64 * t21041 + 0.69644166666666666664e-2_f64 * t21044 + 0.61905925925925925925e-2_f64 * t21048 + 0.20635308641975308642e-2_f64 * t21052 - 0.41270617283950617283e-2_f64 * t15896 + 0.15476481481481481481e-2_f64 * t21055 - 0.11607361111111111111e-2_f64 * t21059 - 0.13345e0_f64 * t1592 * t22723 - 0.2671335375e-1_f64 * t4414 * t22723 + 0.13345e0_f64 * t1592 * t22783 - 0.41270617283950617283e-2_f64 * t21508 + 0.77382407407407407407e-3_f64 * t21512 - 0.23214722222222222221e-2_f64 * t21514 - t17973 + 0.46429444444444444444e-2_f64 * t15941 + 0.890445125e-2_f64 * t4414 * t22759;
    (t22783, t22792)
}
