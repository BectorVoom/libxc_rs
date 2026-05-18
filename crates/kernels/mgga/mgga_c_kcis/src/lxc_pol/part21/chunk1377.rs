//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1377/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1377<F: Float>(t96427: F, t27077: F, t7791: F, t93211: F, t93216: F, t96404: F, t96407: F, t96410: F, t96412: F, t96420: F, t96430: F, t96433: F, t97056: F, t97267: F) -> F {
    let t97465 = F::new(0.23214722222222222222e-2) * t96427;
    let t97470 = -F::new(0.23168402777777777778e-3) * t97267 * t7791 - F::new(0.92835860883789062501e-5) * t27077 * t97056 + F::new(0.23214722222222222222e-2) * t96404 - F::new(0.23214722222222222222e-2) * t96407 + F::new(0.15476481481481481481e-2) * t96410 - F::new(0.25794135802469135802e-3) * t96412 - F::new(0.15476481481481481481e-2) * t96420 + t97465 - F::new(0.92858888888888888886e-2) * t96430 + F::new(0.17024129629629629629e-1) * t96433 + F::new(0.11607361111111111111e-2) * t93211 - F::new(0.61905925925925925926e-2) * t93216;
    t97470
}
