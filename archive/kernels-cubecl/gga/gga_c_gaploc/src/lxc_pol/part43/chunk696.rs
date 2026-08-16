//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 696/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk696<F: Float>(t12953: F, t9272: F, t3177: F, t993: F, t9263: F, t2890: F, t9267: F, t3129: F, t900: F, t10615: F, t9448: F, t986: F) -> (F, F, F, F, F, F, F, F) {
    let t12954 = t9272 * t12953;
    let t12955 = F::cast_from(0.11502877786176224903e1_f64) * t12954;
    let t12957 = t993 * t3177;
    let t12958 = t9263 * t12957;
    let t12960 = t2890 * t3177;
    let t12961 = t9267 * t12960;
    let t12962 = F::cast_from(0.19171462976960374838e1_f64) * t12961;
    let t12968 = t900 * t3129;
    let t12969 = t10615 * t12968;
    let t12986 = t9448 * t986;
    (t12955, t12957, t12958, t12960, t12962, t12968, t12969, t12986)
}
