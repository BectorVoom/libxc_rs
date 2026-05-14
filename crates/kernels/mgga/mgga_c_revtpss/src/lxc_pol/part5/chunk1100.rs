//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1100/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1100<F: Float>(t18950: F, t923: F, t18909: F, t2908: F, t141: F, t18913: F, t11341: F, t18904: F, t18926: F, t930: F, t18930: F, t15169: F, t15170: F, t15189: F, t15192: F, t15198: F, t18944: F) -> (F, F, F, F, F, F, F) {
    let t18951 = t923 * t18950;
    let t18960 = t2908 * t18909;
    let t18961 = t141 * t18960;
    let t18963 = t2908 * t18913;
    let t18964 = t141 * t18963;
    let t18966 = t11341 * t18904;
    let t18967 = t141 * t18966;
    let t18969 = t930 * t18926;
    let t18970 = t141 * t18969;
    let t18972 = t930 * t18930;
    let t18973 = t141 * t18972;
    let t18977 = 0.60385e0 * t18944 + 0.16557e0 * t18961 - 0.5519e-1 * t18964 - 0.36793333333333333333e-1 * t18967 - 0.49671e0 * t18970 + 0.33114e0 * t18973 - t15169 + 0.36793333333333333333e-1 * t15170 - 0.26837777777777777779e0 * t15189 + t15192 + t15198;
    (t18951, t18961, t18964, t18967, t18970, t18973, t18977)
}
