//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 784/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk784<F: Float>(t12709: F, t198: F, t186: F, t561: F, t1019: F, t3399: F, t2790: F, t3451: F, t10326: F, t1006: F, t3445: F, t12705: F, t12707: F, t12713: F, t12715: F, t12719: F, t12721: F, t12725: F, t12726: F, t12728: F, t5933: F, t5944: F, t8440: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12729 = -t12709;
    let t12730 = t198 * t12729;
    let t12731 = t186 * t12730;
    let t12733 = F::new(4.0) / F::new(15.0) * t561 * t12731;
    let t12735 = F::new(4.0) / F::new(5.0) * t3399 * t1019;
    let t12737 = F::new(4.0) / F::new(5.0) * t2790 * t3451;
    let t12739 = F::new(4.0) / F::new(5.0) * t10326 * t3451;
    let t12741 = F::new(2.0) / F::new(5.0) * t1006 * t3445;
    let t12742 = t5933 - t5944 + F::new(8.0) * t8440 - t12705 + t12707 + t12713 + t12715 - t12719 + t12721 + t12725 - t12726 - t12728 + t12733 - t12735 + t12737 + t12739 - t12741;
    (t12729, t12730, t12731, t12733, t12735, t12737, t12739, t12741, t12742)
}
