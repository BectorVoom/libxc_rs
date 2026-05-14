//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1028/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1028<F: Float>(t11990: F, t19196: F, t2597: F, t1086: F, t11790: F, t22581: F, t17760: F, t2580: F, t33273: F, t1: F, t33549: F, t128: F, t18639: F, t941: F, t2660: F, t24759: F, t667: F) -> (F, F, F, F, F, F, F) {
    let t33949 = t11990 * t2597 * t19196;
    let t33952 = t11790 * t1086 * t22581;
    let t33956 = t17760 * t33273 * t2580;
    let t33958 = t33549 * t1;
    let t33961 = t18639 * t941 * t128;
    let t33962 = t2660 * t33958 * t33961;
    let t33965 = t667 * t24759 * M_PI;
    (t33949, t33952, t33956, t33958, t33961, t33962, t33965)
}
