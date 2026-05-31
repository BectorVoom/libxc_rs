//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 697/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk697<F: Float>(t12986: F, t9438: F, t2487: F, t10318: F, t544: F, t9287: F, t10268: F, t2365: F, t4391: F, t3263: F, t8862: F, t2969: F, t3322: F) -> (F, F, F, F, F, F, F, F) {
    let t12987 = t9438 * t12986;
    let t12988 = t2487 * t12987;
    let t12989 = F::cast_from(0.15976219147466979032e-1_f64) * t12988;
    let t12990 = t544 * t10318;
    let t12991 = t12990 * t9287;
    let t12992 = F::cast_from(0.29792074959875355558e-1_f64) * t12991;
    let t12996 = t2365 * t10268;
    let t12997 = t4391 * t12996;
    let t12998 = F::cast_from(0.59584149919750711116e-1_f64) * t12997;
    let t13004 = F::cast_from(2.0_f64) * t8862 * t3263;
    let t13005 = t2969 * t3322;
    (t12987, t12989, t12990, t12992, t12996, t12998, t13004, t13005)
}
