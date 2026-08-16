//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2074/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2074<F: Float>(t2402: F, t973: F, t986: F, t10213: F, t135: F, t41961: F, t697: F, t976: F, t984: F, t13797: F, t10216: F, t343: F) -> (F, F, F, F, F, F, F) {
    let t42903 = t973 * t2402 * t986;
    let t42972 = t135 * t10213;
    let t43002 = F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t41961;
    let t43052 = t697 * t976;
    let t43053 = t43052 * t984;
    let t43069 = t13797 * t984;
    let t43070 = t343 * t10216;
    (t42903, t42972, t43002, t43052, t43053, t43069, t43070)
}
