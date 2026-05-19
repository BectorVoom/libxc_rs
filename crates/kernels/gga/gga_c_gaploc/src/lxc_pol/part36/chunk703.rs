//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 703/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk703<F: Float>(t12986: F, t9438: F, t2487: F, t10318: F, t544: F, t9287: F, t12964: F, t2488: F, t10268: F, t2365: F, t4391: F, t12959: F, t12962: F, t12966: F, t12970: F, t12972: F, t12976: F, t12979: F, t12983: F, t193: F, t557: F, t574: F, t597: F) -> (F, F, F, F, F) {
    let t12987 = t9438 * t12986;
    let t12988 = t2487 * t12987;
    let t12989 = F::cast_from(0.15976219147466979032e-1_f64) * t12988;
    let t12990 = t544 * t10318;
    let t12991 = t12990 * t9287;
    let t12992 = F::cast_from(0.29792074959875355558e-1_f64) * t12991;
    let t12993 = t2488 * t12964;
    let t12994 = t2487 * t12993;
    let t12996 = t2365 * t10268;
    let t12997 = t4391 * t12996;
    let t12998 = F::cast_from(0.59584149919750711116e-1_f64) * t12997;
    let t12999 = -t12959 + t12962 - F::cast_from(0.38342925953920749676e0_f64) * t12966 - t12970 + F::cast_from(0.35750489951850426669e0_f64) * t12972 * t193 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t12976 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t12979 - F::cast_from(0.23005755572352449806e1_f64) * t574 * t12983 + t12989 + t12992 + F::cast_from(0.38342925953920749676e0_f64) * t12994 + t12998;
    (t12987, t12990, t12993, t12996, t12999)
}
