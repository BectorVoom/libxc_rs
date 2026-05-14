//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 273/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk273<F: Float>(t1036: F, t1039: F, t1000: F, t1002: F, t1007: F, t1009: F, t1011: F, t1013: F, t1020: F, t1029: F, t1034: F, t418: F, t995: F, t998: F, t330: F, t363: F) -> (F, F, F) {
    let t1041 = 0.42874018118069736972e-3 * t1036 * t1039;
    let t1042 = -t995 + 0.80031500487063509015e-2 * t998 - 0.40015750243531754508e-2 * t1000 + 0.40015750243531754508e-2 * t1002 - t1007 - 0.17149607247227894789e-2 * t1009 + 0.85748036236139473944e-3 * t1011 - 0.85748036236139473944e-3 * t1013 + 0.12862205435420921092e-2 * t418 * t1020 + 0.42874018118069736972e-2 * t418 * t1029 + t1034 + t1041;
    let t1044 = t330 * t363;
    (t1041, t1042, t1044)
}
