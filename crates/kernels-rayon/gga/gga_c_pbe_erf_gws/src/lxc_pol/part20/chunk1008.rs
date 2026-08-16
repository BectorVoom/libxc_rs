//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1008/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1008(t5621: f64, t987: f64, t101: f64, t510: f64, t981: f64, t5651: f64, t1503: f64, t524: f64, t10063: f64, t10223: f64, t10231: f64, t10233: f64, t10272: f64, t10275: f64, t11168: f64, t11178: f64, t11253: f64, t11270: f64, t125: f64, t143: f64, t169: f64, t2033: f64, t242: f64, t279: f64, t296: f64, t299: f64, t301: f64, t3686: f64, t475: f64, t5670: f64, t5674: f64, t5690: f64, t5694: f64, t5700: f64, t5703: f64, t5707: f64, t5713: f64, t5717: f64, t6028: f64, t6032: f64, t8332: f64, t8355: f64, t8357: f64, t8363: f64, t988: f64) -> f64 {
    let t11274 = t987 * t5621;
    let t11275 = t101 * t11274;
    let t11276 = t981 * t510;
    let t11277 = t5651 * t11276;
    let t11281 = t1503 * t987 * t524;
    let t11285 = 3.0_f64 * t475 * t143 * t10063 - t988 * t10223 + (t5700 - 0.14149184788746388121e0_f64 * t5703 - t5707 - 0.2829836957749277624e0_f64 * t8363 + t8355 + 0.2122377718311958218e0_f64 * t8357 + 0.1061188859155979109e0_f64 * t5713 + t5717 + 0.53059442957798955452e-1_f64 * t10231 - 0.31835665774679373271e-1_f64 * t169 * t10233 * t242 + t10275) * t296 + 0.13559812708347229038e-2_f64 * t5670 + 0.19816831758676854261e0_f64 * t5674 + (t11168 + t11178) * t279 + 0.20267214298646782767e-1_f64 * t169 * t299 * t10272 * t301 + (t11253 + t11270) * t125 - 0.58113483035773838734e-3_f64 * t5690 - t5694 + 2.0_f64 * t11275 * t11277 + 12.0_f64 * t11281 * t8332 - t3686 * t2033 + t6028 - t6032;
    t11285
}
