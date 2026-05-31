//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 959/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk959<F: Float>(t14902: F, t10243: F, t10246: F, t10276: F, t10282: F, t10286: F, t10394: F, t10398: F, t14688: F, t14692: F, t14697: F, t14701: F, t14706: F, t14708: F, t14711: F, t14715: F, t14718: F, t14892: F, t14895: F, t14899: F) -> F {
    let t14903 = t14902 / F::cast_from(9.0_f64);
    let t14904 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14688 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14692 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14697 + t14701 / F::cast_from(3.0_f64) - t14706 + t10394 / F::cast_from(18.0_f64) - t14708 - t10276 / F::cast_from(9.0_f64) - t10246 / F::cast_from(27.0_f64) - t14711 + t10282 / F::cast_from(54.0_f64) + t10286 / F::cast_from(81.0_f64) - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t14715 - F::cast_from(11.0_f64) / F::cast_from(27.0_f64) * t14718 - t10243 / F::cast_from(27.0_f64) - t14892 / F::cast_from(6.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14895 + t14899 / F::cast_from(9.0_f64) + t14903 - t10398;
    t14904
}
