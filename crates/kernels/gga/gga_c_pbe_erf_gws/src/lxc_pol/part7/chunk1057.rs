//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1057/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1057<F: Float>(t19087: F, t475: F, t481: F, t5623: F, t5651: F, t1354: F, t159: F, t285: F, t39: F, t545: F, t5668: F, t143: F, t1593: F, t169: F, t18116: F, t18122: F, t18126: F, t18129: F, t18131: F, t18133: F, t18137: F, t18140: F, t18144: F, t18366: F, t18987: F, t18991: F, t19051: F, t19083: F, t2035: F, t279: F, t296: F, t299: F, t301: F, t522: F, t523: F, t526: F, t5601: F, t5625: F, t5661: F, t5684: F) -> F {
    let t19088 = t475 * t19087;
    let t19090 = t5651 * t5623 * t481;
    let t19098 = t39 * t1354 * t159 * t285;
    let t19101 = t5668 * t545 * t285;
    let t19103 = -F::new(0.21618361918556568284e0) * t18116 - F::new(6.0) * t1593 * t5661 + t18122 + t18126 + F::new(6.0) * t1593 * t5625 - F::new(0.31931290694012290916e0) * t18129 - F::new(0.63862581388024581833e0) * t18131 + F::new(18.0) * t2035 * t18133 + F::new(12.0) * t2035 * t18137 + F::new(36.0) * t5601 * t18140 - t523 * t18144 + (t18366 + t18991) * t279 + t19051 * t296 + F::new(0.20267214298646782767e-1) * t169 * t299 * t18987 * t301 + F::new(3.0) * t475 * t143 * t19083 + F::new(24.0) * t19088 * t19090 + F::new(24.0) * t5684 * t522 * t526 + F::new(0.81358876250083374227e-2) * t19098 + F::new(0.16271775250016674846e-1) * t19101;
    t19103
}
