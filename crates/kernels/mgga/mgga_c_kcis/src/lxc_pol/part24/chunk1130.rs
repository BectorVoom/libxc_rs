//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1130/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1130<F: Float>(t28093: F, t28204: F, t19684: F, t303: F, t356: F, t1014: F, t29000: F, t1856: F, t829: F, t4580: F, t96935: F, t4566: F, t96793: F, t100436: F, t100501: F, t26955: F, t26960: F, t26966: F, t29127: F, t8087: F, t92657: F, t97015: F) -> (F, F, F) {
    let t100834 = t28204 * t28093;
    let t100841 = t303 * t356 * t19684;
    let t100843 = t1014 * t29000;
    let t100845 = t1856 * t829;
    let t100847 = t96935 * t4580 * t100845;
    let t100851 = t96793 * t4566 * t100845;
    let t100862 = 0.30918233506944444445e-4 * t100834 - 0.24734586805555555556e-3 * t97015 * t8087 - 0.92673611111111111112e-3 * t26966 * t29127 - 0.30952962962962962963e-2 * t100841 - 0.25794135802469135802e-3 * t100843 - 0.46336805555555555556e-3 * t26960 * t100847 + 0.30891203703703703704e-3 * t26960 * t100851 - 0.61836467013888888888e-4 * t26955 * t100847 - 0.61890573922526041666e-5 * t92657 * t100501 + 0.41224311342592592592e-4 * t26955 * t100851 - 0.23168402777777777778e-3 * t26960 * t100436;
    (t100841, t100843, t100862)
}
