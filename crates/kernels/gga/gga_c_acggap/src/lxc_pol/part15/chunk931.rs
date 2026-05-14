//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 931/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk931<F: Float>(t34172: F, t34175: F, t34189: F, t34204: F, t34217: F, t34221: F, t34239: F, t34284: F, t34295: F, t34297: F, t34305: F, t34307: F, t34340: F, t34347: F, t34361: F, t34382: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36968 = 0.13719685797782315831e-1 * t34172;
    let t36969 = 0.21437009059034868486e-2 * t34175;
    let t36972 = 0.12579236915841660827e-1 * t34189;
    let t36976 = 0.16006300097412701803e-1 * t34204;
    let t36984 = 0.12579236915841660828e-2 * t34217;
    let t36987 = 0.12579236915841660828e-2 * t34221;
    let t36994 = 0.34299214494455789578e-2 * t34239;
    let t37008 = 0.16006300097412701803e-1 * t34284;
    let t37013 = 0.32012600194825403606e-1 * t34295;
    let t37014 = 0.21437009059034868486e-2 * t34297;
    let t37016 = 0.12579236915841660828e-2 * t34305;
    let t37017 = 0.12862205435420921092e-1 * t34307;
    let t37034 = 0.13719685797782315831e-1 * t34340;
    let t37036 = 0.28582678745379824648e-3 * t34347;
    let t37047 = 0.25724410870841842184e-1 * t34361;
    let t37062 = 7.0 / 72.0 * t34382;
    (t36968, t36969, t36972, t36976, t36984, t36987, t36994, t37008, t37013, t37014, t37016, t37017, t37034, t37036, t37047, t37062)
}
