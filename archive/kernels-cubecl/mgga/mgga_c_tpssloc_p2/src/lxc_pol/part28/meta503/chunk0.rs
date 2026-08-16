//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1739/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1739<F: Float>(t225: F, t7910: F, t26231: F, t26251: F, t26255: F, t26266: F, t22785: F, t22795: F, t26258: F, t26260: F, t26262: F, t26268: F, t26272: F, t26274: F, t26278: F) -> (F, F, F, F) {
    let t27009 = t7910 * t225;
    let t27012 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t26231;
    let t27019 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t26251;
    let t27022 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t26255;
    let t27027 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t26266;
    let t27032 = t27022 - t26258 / F::cast_from(192.0_f64) - t26260 / F::cast_from(192.0_f64) - t26262 / F::cast_from(192.0_f64) + t22785 + F::cast_from(0.40372756094140390853e-3_f64) * t22795 + t27027 + F::cast_from(0.16956557559538964158e-1_f64) * t26268 + F::cast_from(0.40372756094140390853e-3_f64) * t26272 - t26274 / F::cast_from(24.0_f64) - F::cast_from(0.24223653656484234512e-2_f64) * t26278;
    (t27009, t27012, t27019, t27032)
}
