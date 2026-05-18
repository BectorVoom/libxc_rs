//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1103/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1103<F: Float>(t2001: F, t5546: F, t1761: F, t30540: F, t2095: F, t39271: F, t31491: F, t39120: F, t30534: F, t30536: F, t30547: F, t30559: F, t30561: F, t34399: F, t34410: F, t37068: F, t37070: F, t39273: F, t39275: F, t39277: F, t39279: F, t39281: F) -> F {
    let t39283 = t2001 * t5546;
    let t39285 = t30540 * t1761;
    let t39292 = t2095 * t39271;
    let t39294 = t31491 * t39120;
    let t39296 = F::new(0.42874018118069736972e-3) * t39273 - F::new(0.42874018118069736972e-3) * t39275 + F::new(0.47172138434406228102e-3) * t39277 - F::new(0.68598428988911579156e-2) * t39279 + t37068 + F::new(0.68598428988911579156e-2) * t39281 + F::new(0.68598428988911579156e-2) * t39283 - t34399 - t37070 - F::new(0.80031500487063509015e-2) * t39285 - t34410 + F::new(0.95275595817932748827e-3) * t30534 - F::new(0.94344276868812456204e-3) * t30536 - F::new(0.25724410870841842184e-2) * t30547 + F::new(0.41930789719472202756e-3) * t30559 + F::new(0.10482697429868050689e-2) * t30561 - F::new(0.4584375e-1) * t39292 - F::new(0.916875e-1) * t39294;
    t39296
}
