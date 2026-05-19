//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 801/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk801<F: Float>(t34772: F, t34784: F, t34787: F, t34793: F, t34806: F, t34921: F, t2265: F, t3981: F, t35238: F, t8159: F, t874: F, t25809: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t37218 = F::cast_from(0.30487649791575028312e-3_f64) * t34772;
    let t37221 = F::cast_from(0.91462949374725084936e-3_f64) * t34784;
    let t37222 = F::cast_from(0.13010691197123848592e-3_f64) * t34787;
    let t37223 = F::cast_from(0.18292589874945016987e-2_f64) * t34793;
    let t37228 = F::cast_from(0.31113317738916908344e0_f64) * t34806;
    let t37266 = F::cast_from(0.1299607316140891005e-4_f64) * t34921;
    let t37297 = t3981 * t2265;
    let t37375 = F::cast_from(0.91462949374725084936e-3_f64) * t35238;
    let t37393 = t874 * t8159;
    let t37419 = t25809 * t698;
    (t37218, t37221, t37222, t37223, t37228, t37266, t37297, t37375, t37393, t37419)
}
