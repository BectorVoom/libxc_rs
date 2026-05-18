//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1010/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1010<F: Float>(t31312: F, t31316: F, t31322: F, t13287: F, t31057: F, t33953: F, t5122: F, t15386: F, t35284: F, t2302: F, t4210: F, t2260: F, t7852: F) -> (F, F, F, F, F, F, F) {
    let t35538 = F::new(0.85748036236139473944e-3) * t31312;
    let t35539 = F::new(0.12579236915841660827e-2) * t31316;
    let t35541 = F::new(0.85748036236139473944e-3) * t31322;
    let t35549 = t31057 * t13287 * t33953 * t5122;
    let t35552 = t31057 * t15386 * t35284;
    let t35556 = t31057 * t13287 * t2302 * t4210;
    let t35560 = t7852 * t2260;
    (t35538, t35539, t35541, t35549, t35552, t35556, t35560)
}
