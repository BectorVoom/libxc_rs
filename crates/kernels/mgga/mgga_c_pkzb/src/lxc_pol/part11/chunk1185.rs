//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1185/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1185<F: Float>(t10261: F, t11205: F, t179: F, t19026: F, t23278: F, t27119: F, t27122: F, t27151: F, t27153: F, t27155: F, t27175: F, t27178: F, t27181: F, t27232: F, t3026: F, t31086: F, t3174: F, t3235: F, t404: F, t758: F, t824: F, t932: F) -> (F,) {
    let t31892 = 0.25724410870841842184e-2 * t27119 + 0.85748036236139473944e-3 * t27122 - 0.42874018118069736972e-3 * t404 * t179 * t932 * t31086 - 3.0 / 16.0 * t3174 * t23278 * t11205 * t824 + 0.91464571985215438872e-2 * t27151 - 0.91464571985215438872e-2 * t27153 + 0.45732285992607719436e-2 * t27155 - 11.0 / 162.0 * t27175 + 0.25724410870841842183e-2 * t27178 - 0.25724410870841842183e-2 * t27181 - 5.0 / 1296.0 * t19026 - 0.1543464652250510531e-1 * t3235 * t758 * t10261 * t3026 - 0.85748036236139473944e-3 * t27232;
    (t31892,)
}
