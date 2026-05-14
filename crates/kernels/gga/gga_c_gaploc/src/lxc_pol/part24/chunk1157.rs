//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1157/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1157<F: Float>(t33367: F, t11050: F, t11090: F, t2049: F, t28585: F, t32314: F, t33325: F, t33328: F, t33335: F, t33338: F, t33351: F, t33353: F, t33356: F, t33359: F, t33363: F, t33365: F, t3496: F, t531: F, t5669: F, t6021: F, t797: F) -> (F,) {
    let t33368 = 0.89376224879626066674e-1 * t33367;
    let t33369 = -t33325 + t33328 - t33335 - t33338 + 0.1022478025437886658e1 * t5669 * t11050 - 0.35750489951850426669e0 * t797 * t531 * t32314 - 0.23005755572352449806e1 * t6021 * t3496 - 0.71500979903700853338e0 * t2049 * t11090 - t28585 - t33351 - t33353 + t33356 - t33359 - t33363 - t33365 - t33368;
    (t33369,)
}
