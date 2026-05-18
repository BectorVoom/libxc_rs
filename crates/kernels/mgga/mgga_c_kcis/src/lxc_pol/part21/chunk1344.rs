//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1344/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1344<F: Float>(t26955: F, t26960: F, t26977: F, t27020: F, t28204: F, t95569: F, t95579: F, t95581: F, t95585: F, t95626: F, t96779: F, t96781: F, t96787: F, t96790: F, t96795: F, t96799: F) -> F {
    let t96802 = F::new(0.69644166666666666666e-2) * t95569 + t96779 + t96781 + F::new(0.51588271604938271604e-3) * t95579 - F::new(0.41270617283950617284e-2) * t95581 + F::new(0.46377350260416666667e-4) * t28204 * t27020 + F::new(0.46429444444444444443e-2) * t95585 - t96787 - F::new(0.38691203703703703703e-3) * t95626 - F::new(0.92835860883789062501e-5) * t96790 * t26977 + F::new(0.41224311342592592592e-4) * t26955 * t96795 - F::new(0.23168402777777777778e-3) * t26960 * t96799;
    t96802
}
