//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 913/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk913<F: Float>(t5621: F, t987: F, t101: F, t510: F, t981: F, t5651: F, t1503: F, t524: F, t10063: F, t10223: F, t10231: F, t10233: F, t10272: F, t10275: F, t11168: F, t11178: F, t11253: F, t11270: F, t125: F, t143: F, t169: F, t2033: F, t242: F, t279: F, t296: F, t299: F, t301: F, t3686: F, t475: F, t5670: F, t5674: F, t5690: F, t5694: F, t5700: F, t5703: F, t5707: F, t5713: F, t5717: F, t6028: F, t6032: F, t8332: F, t8355: F, t8357: F, t8363: F, t988: F) -> (F,) {
    let t11274 = t987 * t5621;
    let t11275 = t101 * t11274;
    let t11276 = t981 * t510;
    let t11277 = t5651 * t11276;
    let t11281 = t1503 * t987 * t524;
    let t11285 = 3.0 * t475 * t143 * t10063 - t988 * t10223 + (t5700 - 0.14149184788746388121e0 * t5703 - t5707 - 0.2829836957749277624e0 * t8363 + t8355 + 0.2122377718311958218e0 * t8357 + 0.1061188859155979109e0 * t5713 + t5717 + 0.53059442957798955452e-1 * t10231 - 0.31835665774679373271e-1 * t169 * t10233 * t242 + t10275) * t296 + 0.13559812708347229038e-2 * t5670 + 0.19816831758676854261e0 * t5674 + (t11168 + t11178) * t279 + 0.20267214298646782767e-1 * t169 * t299 * t10272 * t301 + (t11253 + t11270) * t125 - 0.58113483035773838734e-3 * t5690 - t5694 + 2.0 * t11275 * t11277 + 12.0 * t11281 * t8332 - t3686 * t2033 + t6028 - t6032;
    (t11285,)
}
