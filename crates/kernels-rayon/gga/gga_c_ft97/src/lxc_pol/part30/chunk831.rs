//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 831/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk831(t206: f64, t35382: f64, t6789: f64, t1419: f64, t27616: f64, t27658: f64, t30671: f64, t33366: f64, t33379: f64, t33380: f64, t33413: f64, t33424: f64, t33426: f64, t35358: f64, t35361: f64, t35368: f64, t35372: f64, t35374: f64, t35376: f64, t35379: f64, t6815: f64, t6833: f64) -> (f64, f64, f64, f64) {
    let t35384 = 1.0_f64 / t206 / t35382;
    let t35385 = t6789 * t35384;
    let t35386 = t35385 * t1419;
    let t35389 = 0.11352761063935582948e-3_f64 * t27658 * t35358 - 0.68246728907663312894e-4_f64 * t33424 * t33426 * t35361 - 0.25537443351851851852e-1_f64 * t33380 * t6833 - t33379 + t33413 - 0.27246626553445399075e-2_f64 * t6815 * t35368 + 4.0_f64 * t35372 - 2.0_f64 * t35374 + 0.89080607335887169333e-3_f64 * t33366 * t35376 - 0.39601100101559655353e-5_f64 * t27616 * t35379 + 0.78129887353338233165e-6_f64 * t30671 * t35386;
    (t35384, t35385, t35386, t35389)
}
