//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 749/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk749<F: Float>(t1419: F, t35367: F, t1410: F, t203: F, t6777: F, t1128: F, t7447: F, t33367: F, t6804: F, t33404: F, t1107: F, t5011: F, t206: F, t6789: F, t27616: F, t27658: F, t30671: F, t33366: F, t33379: F, t33380: F, t33413: F, t33424: F, t33426: F, t35358: F, t35361: F, t6815: F, t6833: F) -> (F, F, F, F, F, F) {
    let t35368 = t35367 * t1419;
    let t35371 = t203 * t1410;
    let t35372 = t35371 * t6777;
    let t35374 = t7447 * t1128;
    let t35376 = t33367 * t6804;
    let t35379 = t33404 * t6804;
    let t35382 = t5011 * t1107;
    let t35384 = 1.0 / t206 / t35382;
    let t35385 = t6789 * t35384;
    let t35386 = t35385 * t1419;
    let t35389 = 0.11352761063935582948e-3 * t27658 * t35358 - 0.68246728907663312894e-4 * t33424 * t33426 * t35361 - 0.25537443351851851852e-1 * t33380 * t6833 - t33379 + t33413 - 0.27246626553445399075e-2 * t6815 * t35368 + 4.0 * t35372 - 2.0 * t35374 + 0.89080607335887169333e-3 * t33366 * t35376 - 0.39601100101559655353e-5 * t27616 * t35379 + 0.78129887353338233165e-6 * t30671 * t35386;
    (t35368, t35371, t35384, t35385, t35386, t35389)
}
