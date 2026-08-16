//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1103/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1103(t2001: f64, t5546: f64, t1761: f64, t30540: f64, t2095: f64, t39271: f64, t31491: f64, t39120: f64, t30534: f64, t30536: f64, t30547: f64, t30559: f64, t30561: f64, t34399: f64, t34410: f64, t37068: f64, t37070: f64, t39273: f64, t39275: f64, t39277: f64, t39279: f64, t39281: f64) -> f64 {
    let t39283 = t2001 * t5546;
    let t39285 = t30540 * t1761;
    let t39292 = t2095 * t39271;
    let t39294 = t31491 * t39120;
    let t39296 = 0.42874018118069736972e-3_f64 * t39273 - 0.42874018118069736972e-3_f64 * t39275 + 0.47172138434406228102e-3_f64 * t39277 - 0.68598428988911579156e-2_f64 * t39279 + t37068 + 0.68598428988911579156e-2_f64 * t39281 + 0.68598428988911579156e-2_f64 * t39283 - t34399 - t37070 - 0.80031500487063509015e-2_f64 * t39285 - t34410 + 0.95275595817932748827e-3_f64 * t30534 - 0.94344276868812456204e-3_f64 * t30536 - 0.25724410870841842184e-2_f64 * t30547 + 0.41930789719472202756e-3_f64 * t30559 + 0.10482697429868050689e-2_f64 * t30561 - 0.4584375e-1_f64 * t39292 - 0.916875e-1_f64 * t39294;
    t39296
}
