//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 984/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk984<F: Float>(t46608: F, t13261: F, t4614: F, t597: F, t10348: F, t3566: F, t1457: F, t1572: F, t44560: F, t46564: F, t46567: F, t46570: F, t46574: F, t46577: F, t46580: F, t46583: F, t46584: F, t46587: F, t46590: F, t46593: F, t46596: F, t46598: F, t46604: F, t46605: F, t46606: F) -> F {
    let t46609 = F::new(0.29792074959875355558e-1) * t46608;
    let t46612 = F::new(0.15337170381568299871e2) * t597 * t4614 * t13261;
    let t46614 = F::new(0.16683561977530199113e1) * t3566 * t10348;
    let t46618 = t46564 + t46567 + t46570 - t46574 - t46577 + t46580 + t46583 - t46584 - t46587 + t46590 - t46593 - t46596 + t46598 - t46604 - t46605 - t46606 + t46609 + t46612 - t46614 + F::new(0.14300195980740170668e1) * t1572 * t1457 * t44560;
    t46618
}
