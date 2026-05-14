//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 682/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk682<F: Float>(t142: F, t5887: F, t1378: F, t1971: F, t5701: F, t4579: F, t550: F, t553: F, t1339: F, t4585: F, t4881: F, t4885: F, t4890: F, t4895: F, t4900: F, t4905: F, t4907: F, t4910: F, t4912: F, t4915: F, t4917: F, t4922: F, t4926: F, t4932: F, t4937: F, t4984: F) -> (F, F, F, F, F) {
    let t5888 = t5887 * t142;
    let t5891 = t5701 * t1378 * t1971;
    let t5895 = 0.59261670986728442646e-2 * t550 * t4579 * t553;
    let t5898 = 0.14862827083471493416e-2 * t1339 * t4585 * t1971;
    let t5899 = -t4881 + t4885 + t4890 - t4895 - t4900 + t4905 + t4907 + t4910 + t4912 + t4915 - t4917 - t4922 + t4926 + t4932 + t4937 + t4984;
    (t5888, t5891, t5895, t5898, t5899)
}
