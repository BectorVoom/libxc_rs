//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 719/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk719<F: Float>(t1944: F, t945: F, t2530: F, t795: F, t740: F, t2042: F, t937: F, t1881: F, t954: F, t2101: F, t935: F, t1891: F, t2095: F, t948: F, t1853: F, t936: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7141 = t1944 * t945;
    let t7143 = t795 * t2530;
    let t7144 = t7143 * t740;
    let t7147 = t2042 * t937;
    let t7152 = t954 * t1881;
    let t7157 = t2101 * t935;
    let t7158 = t7157 * t1891;
    let t7161 = t2095 * t948;
    let t7164 = t936 * t1853;
    (t7141, t7143, t7144, t7147, t7152, t7157, t7158, t7161, t7164)
}
