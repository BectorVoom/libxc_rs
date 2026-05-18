//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 572/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk572<F: Float>(t1356: F, t1360: F, t1387: F, t1413: F, t1418: F, t1421: F, t2052: F, t2059: F, t246: F, t2896: F, t2897: F, t2997: F, t2998: F, t3128: F, t3162: F, t3165: F, t765: F) -> F {
    let t3170 = t2052 - t2059 + t1356 + F::new(0.675260332e-1) * t765 * t3162 + F::new(0.1350520664e0) * t765 * t3165 + t1360 - t2896 + t2897 + t2997 - F::new(0.285764e-1) * t246 * t3128 + t2998 + t1387 + t1413 - t1418 - t1421;
    t3170
}
