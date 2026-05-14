//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1009/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1009<F: Float>(t10007: F, t1710: F, t825: F, t9438: F, t1880: F, t7394: F, t1944: F, t3240: F, t3248: F, t1949: F, t731: F, t9625: F, t21455: F, t739: F, t21446: F, t7211: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29074 = t825 * t9438 * t10007 * t1710;
    let t29078 = t7394 * t9438 * t10007 * t1880;
    let t29160 = 0.19938401337405766662e-2 * t1944 * t3240;
    let t29162 = 0.19938401337405766662e-2 * t1944 * t3248;
    let t29184 = 0.17090058289204942853e-2 * t1949 * t3248;
    let t29186 = 0.17090058289204942853e-2 * t731 * t9625;
    let t29190 = t739 * t21455;
    let t29194 = t739 * t21446;
    let t29210 = 0.64087718584518535698e-3 * t7211 * t3248;
    (t29074, t29078, t29160, t29162, t29184, t29186, t29190, t29194, t29210)
}
