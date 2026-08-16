//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1376/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1376(t30546: f64, t21414: f64, t26773: f64, t3396: f64, t4625: f64, t27071: f64, t544: f64, t9287: f64, t10392: f64, t18337: f64, t10151: f64, t10231: f64, t1391: f64, t1392: f64, t1402: f64, t1429: f64, t2487: f64, t30388: f64, t34386: f64, t34394: f64, t34397: f64, t34404: f64, t34406: f64, t34410: f64, t34414: f64, t34415: f64) -> f64 {
    let t34416 = 0.12780975317973583226e0_f64 * t30546;
    let t34417 = t26773 * t21414;
    let t34418 = 0.29792074959875355558e-1_f64 * t34417;
    let t34419 = t4625 * t3396;
    let t34420 = 0.19171462976960374838e0_f64 * t34419;
    let t34422 = t544 * t27071 * t9287;
    let t34423 = 0.14896037479937677779e-1_f64 * t34422;
    let t34425 = 0.30674340763136599742e1_f64 * t18337 * t10392;
    let t34426 = t30388 - t34386 - 0.92686455430723328401e-1_f64 * t1429 * t1402 * t10231 + 0.11360866949309851756e0_f64 * t2487 * t1391 * t1392 * t10151 - t34394 - t34397 - t34404 - t34406 - t34410 - t34414 - t34415 + t34416 + t34418 - t34420 + t34423 + t34425;
    t34426
}
