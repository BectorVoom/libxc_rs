//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 972/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk972<F: Float>(t140: F, t132: F, t133: F, t139: F, t1683: F, t1698: F, t1992: F, t2043: F, t2058: F, t2072: F, t39802: F, t39803: F, t39807: F, t39813: F, t39818: F, t39824: F, t39828: F, t40051: F, t40099: F, t40128: F, t40164: F, t40193: F, t40226: F, t40258: F, t5785: F, t8825: F, t8852: F, t8859: F, t8895: F) -> F {
    let t141 = F::new(0.1e-59) < t140;
    let t40262 = piecewise3::<f64>(t141, -F::new(12.0) * t1992 * t2072 + F::new(24.0) * t133 * t39802 * t39803 + F::new(6.0) * t133 * t2058 * t39807 + F::new(0.45910941751869106328e2) * t8895 * t1683 + F::new(6.0) * t39813 * t132 * t139 + F::new(0.17516464591774387197e2) * t8859 * t39818 - F::new(0.87582322958871935983e1) * t8852 * t39818 - F::new(0.22445349300913785316e3) * t5785 * t39824 + F::new(0.11222674650456892658e3) * t2043 * t39828 - F::new(0.35032929183548774393e2) * t8825 * t1698 + t40051 + t40099 + t40128 + t40164 + t40193 + t40226 + t40258, F::new(0.0));
    t40262
}
