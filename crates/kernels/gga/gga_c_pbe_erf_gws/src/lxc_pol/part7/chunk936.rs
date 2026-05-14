//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 936/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk936<F: Float>(t18667: F, t18669: F, t18701: F, t18703: F, t18705: F, t18707: F, t18709: F, t18838: F, t18850: F, t18914: F, t18916: F, t18920: F, t18924: F, t1444: F, t4816: F, t1216: F, t1314: F, t470: F, t4734: F, t4737: F) -> (F, F, F) {
    let t18925 = -t18667 - t18669 + t18701 - t18703 + t18705 + t18707 + t18709 + t18914 - t18838 + t18916 + t18850 + t18920 + t18924;
    let t18927 = t4816 * t1444;
    let t18928 = 0.14649244029402527953e-2 * t18927;
    let t18933 = 0.61523382126046769581e4 * t470 * t4734 * t1216 * t4737 * t1314;
    (t18925, t18928, t18933)
}
