//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 984/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk984<F: Float>(t15125: F, t870: F, t2842: F, t4239: F, t10478: F, t1240: F, t2770: F, t4246: F, t309: F, t798: F, t2681: F, t863: F, t848: F, t319: F, t43833: F, t10697: F) -> (F, F, F, F, F, F, F, F, F) {
    let t55792 = t15125 * t870;
    let t55797 = t4239 * t2842;
    let t55937 = t10478 * t1240;
    let t56098 = t2770 * t4246;
    let t56110 = t798 * t309;
    let t56127 = t2681 * t863;
    let t56180 = t848 * t4239;
    let t56339 = t43833 * t319;
    let t56352 = t848 * t10697;
    (t55792, t55797, t55937, t56098, t56110, t56127, t56180, t56339, t56352)
}
