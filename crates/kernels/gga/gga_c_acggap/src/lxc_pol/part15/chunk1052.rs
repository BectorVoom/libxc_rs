//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1052/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1052<F: Float>(t35145: F, t35148: F, t35186: F, t35194: F, t35210: F, t35212: F, t35227: F, t35230: F, t35248: F, t35250: F, t35258: F, t35286: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37408 = F::new(7.0) / F::new(36.0) * t35145;
    let t37409 = F::new(7.0) / F::new(36.0) * t35148;
    let t37428 = F::cast_from(0.12862205435420921092e-1_f64) * t35186;
    let t37430 = F::cast_from(0.37737710747524982482e-2_f64) * t35194;
    let t37437 = F::cast_from(0.18868855373762491241e-1_f64) * t35210;
    let t37438 = F::cast_from(0.42874018118069736972e-2_f64) * t35212;
    let t37442 = F::cast_from(0.28582678745379824648e-2_f64) * t35227;
    let t37443 = F::cast_from(0.34299214494455789578e-2_f64) * t35230;
    let t37450 = F::cast_from(0.85748036236139473944e-2_f64) * t35248;
    let t37451 = F::cast_from(0.32012600194825403606e-1_f64) * t35250;
    let t37458 = F::cast_from(0.32012600194825403606e-1_f64) * t35258;
    let t37475 = F::cast_from(0.85748036236139473944e-3_f64) * t35286;
    (t37408, t37409, t37428, t37430, t37437, t37438, t37442, t37443, t37450, t37451, t37458, t37475)
}
