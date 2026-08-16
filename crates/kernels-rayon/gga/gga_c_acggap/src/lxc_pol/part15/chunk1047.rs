//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1047/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1047(t34217: f64, t34221: f64, t34239: f64, t34284: f64, t34295: f64, t34297: f64, t34305: f64, t34307: f64, t34340: f64, t34347: f64, t34361: f64, t34382: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36984 = 0.12579236915841660828e-2_f64 * t34217;
    let t36987 = 0.12579236915841660828e-2_f64 * t34221;
    let t36994 = 0.34299214494455789578e-2_f64 * t34239;
    let t37008 = 0.16006300097412701803e-1_f64 * t34284;
    let t37013 = 0.32012600194825403606e-1_f64 * t34295;
    let t37014 = 0.21437009059034868486e-2_f64 * t34297;
    let t37016 = 0.12579236915841660828e-2_f64 * t34305;
    let t37017 = 0.12862205435420921092e-1_f64 * t34307;
    let t37034 = 0.13719685797782315831e-1_f64 * t34340;
    let t37036 = 0.28582678745379824648e-3_f64 * t34347;
    let t37047 = 0.25724410870841842184e-1_f64 * t34361;
    let t37062 = 7.0_f64 / 72.0_f64 * t34382;
    (t36984, t36987, t36994, t37008, t37013, t37014, t37016, t37017, t37034, t37036, t37047, t37062)
}
