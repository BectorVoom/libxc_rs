//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 373/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk373<F: Float>(t213: F, t218: F, t215: F, t608: F, t211: F, t408: F, t90: F, t220: F, t612: F, t43: F, t1228: F, t286: F, t708: F, t284: F, zeta_threshold: F) -> (F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t1653 = t215 * t215;
    let t1654 = F::new(1.0) / t1653;
    let t1655 = t608 * t608;
    let t1658 = t211 * t408;
    let t1660 = -F::new(2.0) * t90 + F::new(2.0) * t1658;
    let t1664 = piecewise3::<F>(t214, F::new(0.0), F::new(4.0) / F::new(9.0) * t1654 * t1655 + F::new(4.0) / F::new(3.0) * t215 * t1660);
    let t1665 = t220 * t220;
    let t1666 = F::new(1.0) / t1665;
    let t1667 = t612 * t612;
    let t1670 = -t1660;
    let t1674 = piecewise3::<F>(t219, F::new(0.0), F::new(4.0) / F::new(9.0) * t1666 * t1667 + F::new(4.0) / F::new(3.0) * t220 * t1670);
    let t1676 = (t1664 + t1674) * t43;
    let t1681 = t1228 * t286 * t708;
    let t1683 = t284 * t284;
    (t1676, t1681, t1683)
}
