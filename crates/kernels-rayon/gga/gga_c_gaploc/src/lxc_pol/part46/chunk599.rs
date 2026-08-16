//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 599/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk599(t10374: f64, t1445: f64, t574: f64, t10334: f64, t10336: f64, t10337: f64, t10342: f64, t10345: f64, t10350: f64, t10353: f64, t10356: f64, t10358: f64, t10361: f64, t10363: f64, t10367: f64, t10369: f64, t10373: f64, t1562: f64, t1572: f64, t1646: f64) -> f64 {
    let t10375 = t1445 * t10374;
    let t10377 = 0.46011511144704899612e1_f64 * t574 * t10375;
    let t10378 = t10334 + t10336 - 0.35750489951850426669e0_f64 * t10337 * t1646 - 0.69017266717057349418e1_f64 * t1562 * t10342 + 0.71500979903700853338e0_f64 * t1572 * t10345 - t10350 + t10353 - t10356 - t10358 - t10361 - t10363 - t10367 - t10369 - t10373 - t10377;
    t10378
}
