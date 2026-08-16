//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1189/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1189(t35180: f64, t35186: f64, t35194: f64, t30991: f64, t30993: f64, t30998: f64, t31003: f64, t32664: f64, t32668: f64, t32670: f64, t32671: f64, t32672: f64, t35172: f64, t35176: f64, t35184: f64, t35190: f64, t35198: f64, t35200: f64) -> f64 {
    let t37426 = 0.21437009059034868486e-3_f64 * t35180;
    let t37428 = 0.12862205435420921092e-1_f64 * t35186;
    let t37430 = 0.37737710747524982482e-2_f64 * t35194;
    let t37433 = -t32664 - 0.12862205435420921092e-2_f64 * t30991 - 0.38110238327173099532e-2_f64 * t30993 - 0.41930789719472202758e-3_f64 * t30998 + t32668 - 0.17149607247227894789e-2_f64 * t31003 + t32670 - t32671 + t32672 - 0.12579236915841660828e-2_f64 * t35172 - 0.83861579438944405517e-3_f64 * t35176 + t37426 - 0.41930789719472202758e-3_f64 * t35184 - t37428 + 0.94344276868812456208e-2_f64 * t35190 - t37430 + 0.75475421495049964966e-2_f64 * t35198 + 0.68598428988911579156e-2_f64 * t35200;
    t37433
}
