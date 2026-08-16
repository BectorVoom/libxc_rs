//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 980/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk980(t1445: f64, t2087: f64, t39027: f64, t935: f64, t12218: f64, t2530: f64, t13862: f64, t2197: f64, t47220: f64, t833: f64, t38974: f64, t813: f64) -> (f64, f64, f64, f64, f64) {
    let t47531 = t2087 * t1445 * t39027 * t935;
    let t47535 = t2087 * t1445 * t12218 * t2530;
    let t47537 = t2197 * t13862;
    let t47540 = t833 * t1445 * t47220;
    let t47544 = t813 * t1445 * t38974 * t935;
    (t47531, t47535, t47537, t47540, t47544)
}
