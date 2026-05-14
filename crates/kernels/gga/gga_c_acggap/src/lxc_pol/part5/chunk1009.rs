//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1009/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1009<F: Float>(t1396: F, t1402: F, t1403: F, t1407: F, t150: F, t155: F, t1828: F, t1832: F, t19911: F, t19912: F, t19976: F, t19992: F, t20006: F, t20028: F, t20041: F, t20056: F, t403: F, t4099: F, t4818: F, t5050: F, t5060: F, t5065: F, t5070: F, t5073: F, t5076: F, t519: F, t521: F, t6039: F, t6045: F, t6052: F, t6062: F, t839: F, t917: F, t926: F) -> (F,) {
    let t20084 = 240.0 * t1402 * t5065 * t4818 + 6.0 * t5050 * t521 - (t19911 + t19912 + t19976 + t19992 + t20006 + t20028 + t20041 + t20056) * t150 * t155 + 6.0 * t6039 * t403 - 12.0 * t917 * t1832 + 60.0 * t1402 * t6052 * t839 + 6.0 * t519 * t5076 + 12.0 * t1396 * t1407 - 24.0 * t1402 * t1403 * t4099 - 48.0 * t6045 * t5070 - 24.0 * t5060 * t6062 + 3.0 * t1828 * t926 - 24.0 * t6045 * t5073;
    (t20084,)
}
