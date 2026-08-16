//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 703/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk703(t12986: f64, t9438: f64, t2487: f64, t10318: f64, t544: f64, t9287: f64, t12964: f64, t2488: f64, t10268: f64, t2365: f64, t4391: f64, t12959: f64, t12962: f64, t12966: f64, t12970: f64, t12972: f64, t12976: f64, t12979: f64, t12983: f64, t193: f64, t557: f64, t574: f64, t597: f64) -> (f64, f64, f64, f64, f64) {
    let t12987 = t9438 * t12986;
    let t12988 = t2487 * t12987;
    let t12989 = 0.15976219147466979032e-1_f64 * t12988;
    let t12990 = t544 * t10318;
    let t12991 = t12990 * t9287;
    let t12992 = 0.29792074959875355558e-1_f64 * t12991;
    let t12993 = t2488 * t12964;
    let t12994 = t2487 * t12993;
    let t12996 = t2365 * t10268;
    let t12997 = t4391 * t12996;
    let t12998 = 0.59584149919750711116e-1_f64 * t12997;
    let t12999 = -t12959 + t12962 - 0.38342925953920749676e0_f64 * t12966 - t12970 + 0.35750489951850426669e0_f64 * t12972 * t193 + 0.23005755572352449806e1_f64 * t597 * t12976 - 0.35750489951850426669e0_f64 * t557 * t12979 - 0.23005755572352449806e1_f64 * t574 * t12983 + t12989 + t12992 + 0.38342925953920749676e0_f64 * t12994 + t12998;
    (t12987, t12990, t12993, t12996, t12999)
}
