//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1373/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1373<F: Float>(t30546: F, t21414: F, t26773: F, t3396: F, t4625: F, t27071: F, t544: F, t9287: F, t10392: F, t18337: F, t10151: F, t10231: F, t1391: F, t1392: F, t1402: F, t1429: F, t2487: F, t30388: F, t34386: F, t34394: F, t34397: F, t34404: F, t34406: F, t34410: F, t34414: F, t34415: F) -> F {
    let t34416 = F::new(0.12780975317973583226e0) * t30546;
    let t34417 = t26773 * t21414;
    let t34418 = F::new(0.29792074959875355558e-1) * t34417;
    let t34419 = t4625 * t3396;
    let t34420 = F::new(0.19171462976960374838e0) * t34419;
    let t34422 = t544 * t27071 * t9287;
    let t34423 = F::new(0.14896037479937677779e-1) * t34422;
    let t34425 = F::new(0.30674340763136599742e1) * t18337 * t10392;
    let t34426 = t30388 - t34386 - F::new(0.92686455430723328401e-1) * t1429 * t1402 * t10231 + F::new(0.11360866949309851756e0) * t2487 * t1391 * t1392 * t10151 - t34394 - t34397 - t34404 - t34406 - t34410 - t34414 - t34415 + t34416 + t34418 - t34420 + t34423 + t34425;
    t34426
}
