//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 367/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk367<F: Float>(t1762: F, t257: F, t670: F, t623: F, t78: F, t238: F, t622: F, t233: F, t629: F, t630: F, t1112: F, t1114: F, t1116: F, t1144: F, t1146: F, t1148: F) -> (F, F, F, F, F, F, F) {
    let t1763 = t257 * t1762;
    let t1767 = t670 * t670;
    let t1772 = t78 * t623;
    let t1776 = t622 * t238;
    let t1777 = 1.0 / t1776;
    let t1778 = t233 * t1777;
    let t1779 = t629 * t629;
    let t1780 = t1779 * t630;
    let t1789 = -0.78438333333333333333e0 * t1112 + 0.15687666666666666667e1 * t1114 + 0.68863333333333333333e0 * t1116 + 0.14025833333333333333e0 * t1144 + 0.28051666666666666667e0 * t1146 + 0.17365833333333333333e0 * t1148;
    (t1763, t1767, t1772, t1778, t1779, t1780, t1789)
}
