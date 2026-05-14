//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1062/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1062<F: Float>(t36065: F, t36081: F, t36083: F, t36085: F, t36087: F, t36089: F, t31693: F, t31697: F, t31700: F, t31702: F, t31704: F, t31708: F, t32915: F, t36063: F, t36068: F, t36077: F, t36093: F) -> (F,) {
    let t37848 = 11.0 / 144.0 * t36065;
    let t37857 = 0.12579236915841660828e-2 * t36081;
    let t37858 = 0.42874018118069736972e-3 * t36083;
    let t37859 = 0.21437009059034868486e-2 * t36085;
    let t37860 = 0.85748036236139473944e-3 * t36087;
    let t37861 = 0.42874018118069736972e-3 * t36089;
    let t37863 = t36063 / 24.0 - t37848 + t36068 / 32.0 + 0.28582678745379824648e-2 * t31693 + 0.10718504529517434243e-2 * t31697 - 0.57165357490759649296e-3 * t31700 + 0.62896184579208304138e-3 * t31702 + 0.83861579438944405516e-3 * t31704 + 0.11433071498151929859e-2 * t31708 + 0.37737710747524982482e-2 * t36077 + t37857 - t32915 + t37858 + t37859 + t37860 - t37861 - 0.94344276868812456207e-3 * t36093;
    (t37863,)
}
