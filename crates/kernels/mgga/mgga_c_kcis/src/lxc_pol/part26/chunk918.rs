//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 918/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk918<F: Float>(t15896: F, t1592: F, t15941: F, t17973: F, t21027: F, t21030: F, t21033: F, t21036: F, t21041: F, t21044: F, t21048: F, t21052: F, t21055: F, t21059: F, t21508: F, t21512: F, t21514: F, t22723: F, t22759: F, t22783: F, t4414: F) -> (F,) {
    let t22792 = 0.12381185185185185185e-1 * t21027 - 0.23214722222222222222e-2 * t21030 - 0.34822083333333333332e-2 * t21033 + 0.92858888888888888886e-2 * t21036 - 0.38691203703703703703e-3 * t21041 + 0.69644166666666666664e-2 * t21044 + 0.61905925925925925925e-2 * t21048 + 0.20635308641975308642e-2 * t21052 - 0.41270617283950617283e-2 * t15896 + 0.15476481481481481481e-2 * t21055 - 0.11607361111111111111e-2 * t21059 - 0.13345e0 * t1592 * t22723 - 0.2671335375e-1 * t4414 * t22723 + 0.13345e0 * t1592 * t22783 - 0.41270617283950617283e-2 * t21508 + 0.77382407407407407407e-3 * t21512 - 0.23214722222222222221e-2 * t21514 - t17973 + 0.46429444444444444444e-2 * t15941 + 0.890445125e-2 * t4414 * t22759;
    (t22792,)
}
