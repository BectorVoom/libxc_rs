//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 984/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk984(t46608: f64, t13261: f64, t4614: f64, t597: f64, t10348: f64, t3566: f64, t1457: f64, t1572: f64, t44560: f64, t46564: f64, t46567: f64, t46570: f64, t46574: f64, t46577: f64, t46580: f64, t46583: f64, t46584: f64, t46587: f64, t46590: f64, t46593: f64, t46596: f64, t46598: f64, t46604: f64, t46605: f64, t46606: f64) -> f64 {
    let t46609 = 0.29792074959875355558e-1_f64 * t46608;
    let t46612 = 0.15337170381568299871e2_f64 * t597 * t4614 * t13261;
    let t46614 = 0.16683561977530199113e1_f64 * t3566 * t10348;
    let t46618 = t46564 + t46567 + t46570 - t46574 - t46577 + t46580 + t46583 - t46584 - t46587 + t46590 - t46593 - t46596 + t46598 - t46604 - t46605 - t46606 + t46609 + t46612 - t46614 + 0.14300195980740170668e1_f64 * t1572 * t1457 * t44560;
    t46618
}
