//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1314/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1314(t34411: f64, t6963: f64, t6964: f64, t30542: f64, t30546: f64, t21414: f64, t26773: f64, t3396: f64, t4625: f64, t27071: f64, t544: f64, t9287: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34414 = 0.71500979903700853338e0_f64 * t6963 * t6964 * t34411;
    let t34415 = 0.31952438294933958064e0_f64 * t30542;
    let t34416 = 0.12780975317973583226e0_f64 * t30546;
    let t34417 = t26773 * t21414;
    let t34418 = 0.29792074959875355558e-1_f64 * t34417;
    let t34419 = t4625 * t3396;
    let t34420 = 0.19171462976960374838e0_f64 * t34419;
    let t34422 = t544 * t27071 * t9287;
    (t34414, t34415, t34416, t34418, t34420, t34422)
}
