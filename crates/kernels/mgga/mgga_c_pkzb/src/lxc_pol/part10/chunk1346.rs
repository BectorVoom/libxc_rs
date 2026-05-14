//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1346/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1346<F: Float>(t25910: F, t25913: F, t25918: F, t25921: F, t25924: F, t25927: F, t25930: F, t25933: F, t25936: F, t25939: F, t25943: F, t25946: F, t25949: F, t25953: F, t25956: F, t25959: F, t25964: F, t25967: F, t25971: F, t26027: F, t26282: F, t26285: F, t26356: F, t26364: F, t26366: F, t26369: F, t26371: F, t26374: F) -> (F, F) {
    let t26819 = -t25910 - t25913 + t25918 + t25921 - t25924 - t25927 - t25930 - t25933 - t25936 + t25939 + t25943 + t25946 + t25949 + t25953;
    let t26820 = t25956 - t25959 - t25964 - t25967 - t25971 - t26027 + t26364 - t26282 + t26285 - t26366 + t26369 + t26371 - t26374 + t26356;
    (t26819, t26820)
}
