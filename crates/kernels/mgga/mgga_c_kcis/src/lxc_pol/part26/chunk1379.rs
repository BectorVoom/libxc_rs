//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1379/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1379<F: Float>(t28423: F, t8151: F, t103662: F, t2237: F, t102649: F, t102653: F, t28403: F, t29324: F, t29332: F, t7895: F, t8144: F, t94390: F, t98815: F, t98818: F, t98820: F, t98823: F, t98825: F) -> F {
    let t103693 = t8151 * t28423;
    let t103698 = t2237 * t103662;
    let t103700 = t98815 + t98818 + t98820 - F::new(0.16581944444444444444e-2) * t102649 - F::new(0.185671721767578125e-4) * t94390 * t29324 - F::new(0.13901041666666666667e-2) * t7895 * t29332 - t98823 - F::new(0.12356481481481481481e-2) * t103693 - F::new(0.49745833333333333332e-2) * t102653 + F::new(0.13901041666666666667e-2) * t8144 * t28403 + t98825 - F::new(0.46336805555555555557e-3) * t103698;
    t103700
}
