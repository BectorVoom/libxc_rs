//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 887/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk887<F: Float>(t21502: F, t42944: F, t1841: F, t21501: F, t13182: F, t2563: F, t3487: F, t7284: F, t9647: F, t29277: F, t32607: F, t10639: F, t16879: F, t883: F) -> (F, F, F, F, F, F) {
    let t42945 = t21502 * t42944;
    let t42948 = F::new(0.51270174867614828557e-2) * t1841 * t21501 * t42945;
    let t42953 = t1841 * t13182;
    let t42954 = F::new(0.85450291446024714264e-3) * t42953;
    let t42960 = t9647 * t7284 * t3487 * t2563;
    let t42961 = F::new(0.4486140300916297499e-2) * t42960;
    let t42963 = t9647 * t29277 * t32607;
    let t42967 = t9647 * t16879 * t883 * t10639;
    (t42945, t42948, t42954, t42961, t42963, t42967)
}
