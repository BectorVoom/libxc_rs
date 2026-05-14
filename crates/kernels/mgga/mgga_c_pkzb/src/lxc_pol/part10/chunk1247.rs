//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1247/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1247<F: Float>(t16489: F, t16493: F, t16497: F, t16513: F, t16517: F, t16526: F, t16531: F, t16536: F, t23924: F, t23925: F, t23929: F, t23933: F, t23934: F, t23935: F, t23940: F, t23941: F, t23943: F, t23944: F) -> (F,) {
    let t24525 = -t16489 - t16493 + t16497 - t23924 - t23925 - t23929 - t23933 + t23934 + t23935 - t16513 + t16517 + t23940 - t23941 + t16526 + t23943 + t16531 + t23944 + t16536;
    (t24525,)
}
