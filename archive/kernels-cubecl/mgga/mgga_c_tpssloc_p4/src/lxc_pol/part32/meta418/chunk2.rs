//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1618/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1618<F: Float>(t1196: F, t16558: F, t974: F, t1215: F, t1653: F, t15659: F, t3578: F, t1177: F, t18221: F, t18237: F, t1735: F, t4724: F) -> (F, F, F, F, F) {
    let t18996 = t1196 * t16558;
    let t18997 = t974 * t18996;
    let t19000 = t1653 * t1215;
    let t19001 = t15659 * t19000;
    let t19002 = t3578 * t19001;
    let t19005 = t1177 * t18221;
    let t19010 = t1177 * t18237;
    let t19015 = t1735 * t4724;
    (t18997, t19002, t19005, t19010, t19015)
}
