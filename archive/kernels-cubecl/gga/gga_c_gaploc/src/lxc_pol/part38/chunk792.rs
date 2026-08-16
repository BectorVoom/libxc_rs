//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 792/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk792<F: Float>(t12964: F, t587: F, t589: F, t1429: F, t2365: F, t2366: F, t31747: F, t34777: F, t901: F, t35106: F, t40261: F, t10418: F, t2389: F) -> (F, F, F, F, F, F) {
    let t41954 = t587 * t589 * t12964;
    let t41958 = t1429 * t2365 * t2366 * t31747;
    let t41960 = t34777 * t901;
    let t41962 = t35106 * t901;
    let t41981 = F::cast_from(0.12780975317973583225e0_f64) * t40261;
    let t42001 = t10418 * t2389;
    (t41954, t41958, t41960, t41962, t41981, t42001)
}
