//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 726/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk726<F: Float>(t3885: F, t9853: F, t2606: F, t9723: F, t9727: F, t9735: F, t9701: F, t9730: F, t9520: F, t9695: F, t9705: F, t9711: F, t9715: F, t9720: F, t9739: F, t9752: F) -> (F, F, F) {
    let t9854 = t3885 * t9853;
    let t9855 = t2606 * t9854;
    let t9861 = t9723 / F::cast_from(9.0_f64);
    let t9862 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9727;
    let t9863 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9735;
    let t9867 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9701;
    let t9869 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9730;
    let t9870 = t9520 / F::cast_from(3.0_f64);
    let t9871 = -t9705 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) * t9715 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t9720 + t9861 + t9862 - t9863 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9739 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9752 - t9695 / F::cast_from(3.0_f64) - t9867 - F::cast_from(2.0_f64) * t9711 - t9869 + t9870;
    (t9854, t9855, t9871)
}
