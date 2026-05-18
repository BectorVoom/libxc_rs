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
    let t14903 = t14902 / F::new(9.0);
    let t14904 = F::new(2.0) / F::new(27.0) * t14688 - F::new(2.0) / F::new(9.0) * t14692 + F::new(2.0) / F::new(3.0) * t14697 + t14701 / F::new(3.0) - t14706 + t10394 / F::new(18.0) - t14708 - t10276 / F::new(9.0) - t10246 / F::new(27.0) - t14711 + t10282 / F::new(54.0) + t10286 / F::new(81.0) - F::new(2.0) / F::new(81.0) * t14715 - F::new(11.0) / F::new(27.0) * t14718 - t10243 / F::new(27.0) - t14892 / F::new(6.0) - F::new(2.0) / F::new(27.0) * t14895 + t14899 / F::new(9.0) + t14903 - t10398;
    t14904
}
