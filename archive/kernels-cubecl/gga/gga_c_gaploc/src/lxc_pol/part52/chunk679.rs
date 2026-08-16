//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 679/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk679<F: Float>(t9448: F, t986: F, t9438: F, t2487: F, t10318: F, t544: F, t9287: F, t10268: F, t2365: F, t4391: F, t2097: F, t3039: F) -> (F, F, F, F, F, F, F, F) {
    let t12986 = t9448 * t986;
    let t12987 = t9438 * t12986;
    let t12988 = t2487 * t12987;
    let t12990 = t544 * t10318;
    let t12991 = t12990 * t9287;
    let t12996 = t2365 * t10268;
    let t12997 = t4391 * t12996;
    let t13045 = t3039 * t2097;
    (t12986, t12987, t12988, t12990, t12991, t12996, t12997, t13045)
}
