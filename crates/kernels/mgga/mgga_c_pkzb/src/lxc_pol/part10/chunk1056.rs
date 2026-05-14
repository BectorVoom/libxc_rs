//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1056/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1056<F: Float>(t1676: F, t3501: F, t1535: F, t5025: F, t5028: F, t5040: F, t5066: F, t5069: F, t5073: F, t5186: F, t5324: F, t5333: F, t5338: F, t5344: F, t568: F, t8845: F, t8846: F, t8848: F, t8849: F, t8851: F, t8853: F, t8854: F, t8855: F) -> (F, F) {
    let t9121 = t3501 * t1676;
    let t9125 = -3.0 * t1535 * t568 * t9121 + t5025 + t5028 + t5040 + t5066 - t5069 - t5073 + t5186 - t5324 + t5333 - t5338 - t5344 + t8845 + t8846 + t8848 + t8849 + t8851 - t8853 + t8854 - t8855;
    (t9121, t9125)
}
