//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 610/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk610<F: Float>(t12938: F, t9438: F, t587: F, t2366: F, t3338: F, t2365: F, t1429: F, t10418: F, t901: F, t10608: F, t3177: F, t9272: F, t993: F, t9263: F, t2890: F, t9267: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12939 = t9438 * t12938;
    let t12940 = t587 * t12939;
    let t12942 = t2366 * t3338;
    let t12943 = t2365 * t12942;
    let t12944 = t1429 * t12943;
    let t12946 = t10418 * t901;
    let t12953 = t10608 * t3177;
    let t12954 = t9272 * t12953;
    let t12957 = t993 * t3177;
    let t12958 = t9263 * t12957;
    let t12960 = t2890 * t3177;
    let t12961 = t9267 * t12960;
    (t12939, t12940, t12942, t12943, t12944, t12946, t12953, t12954, t12957, t12958, t12960, t12961)
}
