//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 613/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk613<F: Float>(t9439: F, t986: F, t9438: F, t587: F, t10608: F, t3177: F, t9272: F, t993: F, t9263: F, t2890: F, t9267: F, t3129: F, t900: F, t10615: F, t9448: F, t2487: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12938 = t9439 * t986;
    let t12939 = t9438 * t12938;
    let t12940 = t587 * t12939;
    let t12953 = t10608 * t3177;
    let t12954 = t9272 * t12953;
    let t12957 = t993 * t3177;
    let t12958 = t9263 * t12957;
    let t12960 = t2890 * t3177;
    let t12961 = t9267 * t12960;
    let t12968 = t900 * t3129;
    let t12969 = t10615 * t12968;
    let t12986 = t9448 * t986;
    let t12987 = t9438 * t12986;
    let t12988 = t2487 * t12987;
    (t12938, t12939, t12940, t12953, t12954, t12957, t12958, t12960, t12961, t12968, t12969, t12986, t12987, t12988)
}
