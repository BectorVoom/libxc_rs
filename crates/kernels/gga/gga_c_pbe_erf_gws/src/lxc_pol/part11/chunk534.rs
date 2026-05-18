//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 534/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk534<F: Float>(t3591: F, t3592: F, t3606: F, t3607: F, t153: F, t156: F, t1596: F, t1601: F, t1608: F, t1611: F, t168: F, t1937: F, t242: F, t245: F, t2520: F, t2526: F, t2531: F, t2837: F, t3373: F, t3380: F) -> (F, F) {
    let t3609 = t3591 + t3592 + t3606 + t3607;
    let t3617 = -t1596 + F::new(0.16752564107100880375e0) * t2520 + t1601 - F::new(0.83762820535504401876e-1) * t3380 * t242 - F::new(0.16752564107100880375e0) * t2526 - t1608 - t1611 + F::new(0.39794582218349216586e-1) * t2531 - F::new(0.11938374665504764976e-1) * t168 * t245 * t3609 + t1937 - F::new(0.11389037339096724978e1) * t2837 + F::new(0.42708890021612718669e0) * t153 * t156 * t3373;
    (t3609, t3617)
}
