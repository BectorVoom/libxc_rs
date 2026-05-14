//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1011/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1011<F: Float>(t10024: F, t1980: F, t7442: F, t2586: F, t4752: F, t10007: F, t1710: F, t825: F, t9438: F, t1880: F, t7394: F, t1944: F, t3240: F, t3248: F, t1949: F, t731: F, t9625: F) -> (F, F, F, F, F, F, F, F) {
    let t29035 = 0.17875244975925213335e0 * t1980 * t7442 * t10024;
    let t29052 = t4752 * t2586;
    let t29074 = t825 * t9438 * t10007 * t1710;
    let t29078 = t7394 * t9438 * t10007 * t1880;
    let t29160 = 0.19938401337405766662e-2 * t1944 * t3240;
    let t29162 = 0.19938401337405766662e-2 * t1944 * t3248;
    let t29184 = 0.17090058289204942853e-2 * t1949 * t3248;
    let t29186 = 0.17090058289204942853e-2 * t731 * t9625;
    (t29035, t29052, t29074, t29078, t29160, t29162, t29184, t29186)
}
