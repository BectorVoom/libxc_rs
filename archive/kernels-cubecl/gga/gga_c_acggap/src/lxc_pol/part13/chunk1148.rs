//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1148/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1148<F: Float>(t31593: F, t31544: F, t31565: F, t31570: F, t31585: F, t35731: F, t35733: F, t35737: F, t35738: F, t35740: F, t35742: F, t35744: F, t35748: F, t35751: F, t35753: F, t35756: F, t35759: F) -> F {
    let t35764 = F::cast_from(0.42874018118069736972e-3_f64) * t31593;
    let t35765 = F::cast_from(0.34299214494455789578e-2_f64) * t35731 - F::cast_from(0.85748036236139473944e-3_f64) * t35733 + F::cast_from(0.66040993808168719343e-1_f64) * t31544 - t35737 + F::cast_from(0.34299214494455789578e-2_f64) * t35738 + F::cast_from(0.80031500487063509014e-2_f64) * t35740 - F::cast_from(0.34299214494455789578e-2_f64) * t35742 - F::cast_from(0.12862205435420921092e-2_f64) * t35744 - t35748 - F::cast_from(0.21437009059034868486e-2_f64) * t35751 - F::cast_from(0.68598428988911579156e-2_f64) * t35753 + t35756 - F::cast_from(0.7862023072401038017e-3_f64) * t35759 + F::cast_from(0.31448092289604152068e-3_f64) * t31565 + F::cast_from(0.62896184579208304136e-3_f64) * t31570 + F::cast_from(0.10718504529517434243e-3_f64) * t31585 - t35764;
    t35765
}
