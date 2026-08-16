//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1207/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1207(t30106: f64, t32348: f64, t32349: f64, t32350: f64, t32352: f64, t33894: f64, t33908: f64, t33916: f64, t33922: f64, t33927: f64, t36841: f64, t38820: f64, t38830: f64, t38834: f64, t38840: f64, t38846: f64, t38848: f64, t38852: f64) -> f64 {
    let t41371 = 0.25724410870841842183e-2_f64 * t38820 - 0.57165357490759649296e-3_f64 * t33894 - t36841 - t32348 - t32349 + t32350 + t32352 + t33908 - 0.12579236915841660828e-2_f64 * t33916 + 0.37737710747524982482e-2_f64 * t30106 + t33922 - t33927 - 0.42874018118069736972e-3_f64 * t38830 - 0.21437009059034868486e-3_f64 * t38834 - 0.12862205435420921092e-2_f64 * t38840 + 0.94344276868812456204e-2_f64 * t38846 - 0.13719685797782315831e-1_f64 * t38848 - 0.31448092289604152068e-2_f64 * t38852;
    t41371
}
