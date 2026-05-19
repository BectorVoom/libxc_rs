//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 599/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk599<F: Float>(t10374: F, t1445: F, t574: F, t10334: F, t10336: F, t10337: F, t10342: F, t10345: F, t10350: F, t10353: F, t10356: F, t10358: F, t10361: F, t10363: F, t10367: F, t10369: F, t10373: F, t1562: F, t1572: F, t1646: F) -> F {
    let t10375 = t1445 * t10374;
    let t10377 = F::cast_from(0.46011511144704899612e1_f64) * t574 * t10375;
    let t10378 = t10334 + t10336 - F::cast_from(0.35750489951850426669e0_f64) * t10337 * t1646 - F::cast_from(0.69017266717057349418e1_f64) * t1562 * t10342 + F::cast_from(0.71500979903700853338e0_f64) * t1572 * t10345 - t10350 + t10353 - t10356 - t10358 - t10361 - t10363 - t10367 - t10369 - t10373 - t10377;
    t10378
}
