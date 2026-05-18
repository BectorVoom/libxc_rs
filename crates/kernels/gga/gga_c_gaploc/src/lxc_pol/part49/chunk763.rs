//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 763/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk763<F: Float>(t3129: F, t900: F, t10615: F, t9448: F, t986: F, t9438: F, t2487: F, t10318: F, t544: F, t9287: F, t12964: F, t2488: F) -> (F, F, F, F, F, F, F, F) {
    let t12968 = t900 * t3129;
    let t12969 = t10615 * t12968;
    let t12970 = F::new(0.89376224879626066675e-1) * t12969;
    let t12986 = t9448 * t986;
    let t12987 = t9438 * t12986;
    let t12988 = t2487 * t12987;
    let t12989 = F::new(0.15976219147466979032e-1) * t12988;
    let t12990 = t544 * t10318;
    let t12991 = t12990 * t9287;
    let t12992 = F::new(0.29792074959875355558e-1) * t12991;
    let t12993 = t2488 * t12964;
    (t12968, t12970, t12986, t12987, t12989, t12990, t12992, t12993)
}
