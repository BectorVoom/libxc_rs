//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1268/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1268<F: Float>(t10: F, t17: F, t23323: F, t3021: F, t23329: F, t3: F, t19735: F, t19737: F, t19739: F, t19744: F, t23253: F, t23255: F, t23257: F, t23284: F, t23311: F, t23328: F, t23335: F, t23355: F) -> (F, F) {
    let t27275 = t23323 * t10 * t3021 * t17;
    let t27276 = t23329 * t3;
    let t27288 = F::new(28.0) / F::new(729.0) * t19735 - F::new(2.0) / F::new(243.0) * t19737 - F::new(4.0) / F::new(729.0) * t19739 + F::new(4.0) / F::new(243.0) * t19744 + F::new(2.0) / F::new(81.0) * t23253 - F::new(4.0) / F::new(81.0) * t23255 + F::new(2.0) / F::new(27.0) * t23257 - F::new(40.0) / F::new(243.0) * t27275 * t23328 * t27276 + F::new(16.0) / F::new(27.0) * t27275 * t23335 * t27276 - F::new(8.0) / F::new(9.0) * t27275 * t23355 * t27276 - F::new(16.0) / F::new(729.0) * t23284 + F::new(2.0) / F::new(243.0) * t23311;
    (t27275, t27288)
}
