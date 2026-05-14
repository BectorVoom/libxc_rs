//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1082/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1082<F: Float>(t2395: F, t52385: F, t13469: F, t2418: F, t52263: F, t52267: F, t6832: F, t96615: F, t505: F, t709: F, t24360: F, t3766: F, t27529: F, t27609: F, t697: F, t52900: F, t6757: F) -> (F, F, F, F, F, F, F, F, F) {
    let t108460 = t52385 * t2395;
    let t108464 = t13469 * t2418;
    let t108468 = t52263 * t2395;
    let t108472 = t52267 * t2395;
    let t108476 = t96615 * t6832;
    let t108479 = t505 * t709;
    let t108487 = t3766 * t24360;
    let t108494 = 0.29693535778629056444e-3 * t27609 * t697 * t27529;
    let t108495 = t6757 * t52900;
    (t108460, t108464, t108468, t108472, t108476, t108479, t108487, t108494, t108495)
}
