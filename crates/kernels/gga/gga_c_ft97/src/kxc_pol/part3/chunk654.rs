//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 654/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk654<F: Float>(t1882: F, t4164: F, t4169: F, t12001: F, t4159: F, t4241: F, t681: F, t89: F, t1240: F, t2770: F, t848: F, t4305: F, t319: F, t871: F, t4248: F, t4301: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15168 = 4.0 / 9.0 * t1882 * t4164;
    let t15170 = 2.0 / 9.0 * t1882 * t4169;
    let t15180 = t12001 * t4159;
    let t15190 = 2.0 / 9.0 * t89 * t681 * t4241;
    let t15191 = t2770 * t1240;
    let t15195 = t848 * t1240;
    let t15206 = 2.0 / 9.0 * t1882 * t4305;
    let t15229 = t2770 * t319;
    let t15254 = t848 * t871;
    let t15271 = 2.0 / 9.0 * t1882 * t4248;
    let t15273 = 2.0 / 9.0 * t1882 * t4301;
    (t15168, t15170, t15180, t15190, t15191, t15195, t15206, t15229, t15254, t15271, t15273)
}
