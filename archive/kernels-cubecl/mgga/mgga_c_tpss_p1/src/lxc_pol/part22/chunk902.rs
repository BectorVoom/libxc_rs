//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 902/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk902<F: Float>(t7813: F, t7875: F, t7878: F, t735: F, t1985: F, t725: F, t2337: F, t2428: F, t823: F, t200: F, t45: F, t202: F, t57: F) -> (F, F, F, F, F, F) {
    let t8038 = t7875 * t7813 * t7878;
    let t8040 = F::cast_from(0.10254018858216406658e4_f64) * t735 * t8038;
    let t8042 = t725 * t1985;
    let t8043 = t2337 * t8042;
    let t8045 = t2428 * t823;
    let t8050 = F::cast_from(1.0_f64) / t200 / t45;
    let t8061 = F::cast_from(1.0_f64) / t202 / t57;
    (t8038, t8040, t8043, t8045, t8050, t8061)
}
