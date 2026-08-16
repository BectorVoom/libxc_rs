//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1125/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1125<F: Float>(t22754: F, t22757: F, t22762: F, t22767: F, t22768: F, t22771: F, t22774: F, t22777: F, t22780: F, t22785: F, t22786: F, t22789: F, t22795: F, t22799: F, t22800: F) -> F {
    let t22802 = -t22754 / F::cast_from(1536.0_f64) - t22757 / F::cast_from(768.0_f64) + t22762 / F::cast_from(768.0_f64) + t22767 - t22768 / F::cast_from(1536.0_f64) - F::cast_from(0.20186378047070195427e-3_f64) * t22771 - F::cast_from(0.20186378047070195427e-3_f64) * t22774 + F::cast_from(0.40372756094140390854e-3_f64) * t22777 + F::cast_from(0.28260929265898273598e-2_f64) * t22780 + t22785 - t22786 / F::cast_from(384.0_f64) - t22789 / F::cast_from(192.0_f64) + F::cast_from(0.40372756094140390854e-3_f64) * t22795 + t22799 - t22800 / F::cast_from(48.0_f64);
    t22802
}
