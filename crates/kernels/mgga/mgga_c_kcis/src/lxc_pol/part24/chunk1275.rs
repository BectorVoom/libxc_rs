//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1275/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1275<F: Float>(t100436: F, t100501: F, t100834: F, t100841: F, t100843: F, t100847: F, t100851: F, t26955: F, t26960: F, t26966: F, t29127: F, t8087: F, t92657: F, t97015: F) -> F {
    let t100862 = F::new(0.30918233506944444445e-4) * t100834 - F::new(0.24734586805555555556e-3) * t97015 * t8087 - F::new(0.92673611111111111112e-3) * t26966 * t29127 - F::new(0.30952962962962962963e-2) * t100841 - F::new(0.25794135802469135802e-3) * t100843 - F::new(0.46336805555555555556e-3) * t26960 * t100847 + F::new(0.30891203703703703704e-3) * t26960 * t100851 - F::new(0.61836467013888888888e-4) * t26955 * t100847 - F::new(0.61890573922526041666e-5) * t92657 * t100501 + F::new(0.41224311342592592592e-4) * t26955 * t100851 - F::new(0.23168402777777777778e-3) * t26960 * t100436;
    t100862
}
