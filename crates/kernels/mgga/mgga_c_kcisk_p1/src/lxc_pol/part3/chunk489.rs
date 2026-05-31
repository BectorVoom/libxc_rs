//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 489/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk489<F: Float>(t3841: F, t435: F, t437: F, t3042: F, t41: F, t447: F, t445: F, t1394: F, t429: F, t431: F, t3812: F, t1056: F, t443: F) -> (F, F, F, F, F, F) {
    let t3844 = F::cast_from(0.21133333333333333333e-2_f64) * t435 * t3841 * t437;
    let t3845 = t3042 * t41;
    let t3846 = t3845 * t447;
    let t3848 = F::cast_from(0.16804375e-4_f64) * t445 * t3846;
    let t3851 = F::cast_from(0.8197e-2_f64) * t429 * t1394 * t431;
    let t3852 = F::cast_from(0.23911438650126355246e-1_f64) * t3812;
    let t3853 = t443 * t1056;
    (t3844, t3845, t3848, t3851, t3852, t3853)
}
