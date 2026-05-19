//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 380/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk380<F: Float>(t238: F, t622: F, t233: F, t629: F, t630: F, t1112: F, t1114: F, t1116: F, t1144: F, t1146: F, t1148: F, t241: F) -> (F, F, F, F, F, F) {
    let t1776 = t622 * t238;
    let t1777 = F::new(1.0) / t1776;
    let t1778 = t233 * t1777;
    let t1779 = t629 * t629;
    let t1780 = t1779 * t630;
    let t1789 = -F::cast_from(0.78438333333333333333e0_f64) * t1112 + F::cast_from(0.15687666666666666667e1_f64) * t1114 + F::cast_from(0.68863333333333333333e0_f64) * t1116 + F::cast_from(0.14025833333333333333e0_f64) * t1144 + F::cast_from(0.28051666666666666667e0_f64) * t1146 + F::cast_from(0.17365833333333333333e0_f64) * t1148;
    let t1790 = t1789 * t630;
    let t1793 = t622 * t622;
    let t1794 = F::new(1.0) / t1793;
    let t1795 = t233 * t1794;
    let t1796 = t241 * t241;
    (t1778, t1779, t1780, t1790, t1795, t1796)
}
