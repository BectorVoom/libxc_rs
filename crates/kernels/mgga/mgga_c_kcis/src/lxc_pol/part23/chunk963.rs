//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 963/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk963<F: Float>(t12581: F, t2256: F, t2260: F, t27333: F, t27335: F, t27337: F, t27362: F, t27366: F, t27556: F, t27560: F, t27564: F, t27567: F, t27569: F, t7968: F, t7971: F, t7978: F) -> (F, F) {
    let t27575 = t12581 * t2256;
    let t27582 = 0.92754700520833333334e-4 * t27556 * t7971 + 0.46377350260416666667e-4 * t7968 * t27560 + 0.30918233506944444444e-4 * t27564 + 0.30918233506944444444e-4 * t27567 * t27569 + 0.34822083333333333332e-2 * t27333 - 0.23214722222222222222e-2 * t27335 + 0.15476481481481481481e-2 * t27337 - 0.34752604166666666667e-3 * t27575 * t2260 + 0.34752604166666666667e-3 * t7978 * t27560 + 0.15476481481481481481e-2 * t27362 + 0.23214722222222222222e-2 * t27366;
    (t27575, t27582)
}
