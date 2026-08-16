//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1344/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1344<F: Float>(t63907: F, t63913: F, t63917: F, t63899: F, t63901: F, t63903: F, t63905: F, t63909: F, t63911: F, t63921: F, t63923: F, t63925: F) -> F {
    let t66390 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t63907;
    let t66393 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t63913;
    let t66394 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t63917;
    let t66398 = -t63899 / F::cast_from(384.0_f64) - t63901 / F::cast_from(768.0_f64) + t63903 / F::cast_from(96.0_f64) + t63905 / F::cast_from(192.0_f64) - t66390 + t63909 / F::cast_from(192.0_f64) + t63911 / F::cast_from(96.0_f64) - t66393 - t66394 - t63921 / F::cast_from(128.0_f64) + t63923 / F::cast_from(128.0_f64) - t63925 / F::cast_from(768.0_f64);
    t66398
}
