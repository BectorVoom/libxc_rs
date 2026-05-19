//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 831/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk831<F: Float>(t206: F, t35382: F, t6789: F, t1419: F, t27616: F, t27658: F, t30671: F, t33366: F, t33379: F, t33380: F, t33413: F, t33424: F, t33426: F, t35358: F, t35361: F, t35368: F, t35372: F, t35374: F, t35376: F, t35379: F, t6815: F, t6833: F) -> (F, F, F, F) {
    let t35384 = F::new(1.0) / t206 / t35382;
    let t35385 = t6789 * t35384;
    let t35386 = t35385 * t1419;
    let t35389 = F::cast_from(0.11352761063935582948e-3_f64) * t27658 * t35358 - F::cast_from(0.68246728907663312894e-4_f64) * t33424 * t33426 * t35361 - F::cast_from(0.25537443351851851852e-1_f64) * t33380 * t6833 - t33379 + t33413 - F::cast_from(0.27246626553445399075e-2_f64) * t6815 * t35368 + F::new(4.0) * t35372 - F::new(2.0) * t35374 + F::cast_from(0.89080607335887169333e-3_f64) * t33366 * t35376 - F::cast_from(0.39601100101559655353e-5_f64) * t27616 * t35379 + F::cast_from(0.78129887353338233165e-6_f64) * t30671 * t35386;
    (t35384, t35385, t35386, t35389)
}
