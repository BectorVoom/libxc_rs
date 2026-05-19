//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1384/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1384<F: Float>(t6188: F, t6207: F, t15896: F, t1592: F, t15941: F, t17973: F, t21027: F, t21030: F, t21033: F, t21036: F, t21041: F, t21044: F, t21048: F, t21052: F, t21055: F, t21059: F, t21508: F, t21512: F, t21514: F, t22723: F, t22759: F, t4414: F) -> (F, F) {
    let t22783 = t6207 * t6188;
    let t22792 = F::cast_from(0.12381185185185185185e-1_f64) * t21027 - F::cast_from(0.23214722222222222222e-2_f64) * t21030 - F::cast_from(0.34822083333333333332e-2_f64) * t21033 + F::cast_from(0.92858888888888888886e-2_f64) * t21036 - F::cast_from(0.38691203703703703703e-3_f64) * t21041 + F::cast_from(0.69644166666666666664e-2_f64) * t21044 + F::cast_from(0.61905925925925925925e-2_f64) * t21048 + F::cast_from(0.20635308641975308642e-2_f64) * t21052 - F::cast_from(0.41270617283950617283e-2_f64) * t15896 + F::cast_from(0.15476481481481481481e-2_f64) * t21055 - F::cast_from(0.11607361111111111111e-2_f64) * t21059 - F::new(0.13345e0) * t1592 * t22723 - F::cast_from(0.2671335375e-1_f64) * t4414 * t22723 + F::new(0.13345e0) * t1592 * t22783 - F::cast_from(0.41270617283950617283e-2_f64) * t21508 + F::cast_from(0.77382407407407407407e-3_f64) * t21512 - F::cast_from(0.23214722222222222221e-2_f64) * t21514 - t17973 + F::cast_from(0.46429444444444444444e-2_f64) * t15941 + F::cast_from(0.890445125e-2_f64) * t4414 * t22759;
    (t22783, t22792)
}
