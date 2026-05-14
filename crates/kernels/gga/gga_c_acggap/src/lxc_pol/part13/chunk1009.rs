//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1009/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1009<F: Float>(t35646: F, t31477: F, t171: F, t5011: F, t31443: F, t35296: F, t31479: F, t1017: F, t2030: F, t2297: F, t8927: F, t2288: F, t4262: F, t7450: F, t922: F, t2310: F, t7780: F) -> (F, F, F, F, F, F, F) {
    let t35647 = 0.1528125e-1 * t35646;
    let t35648 = 0.13073958333333333333e0 * t31477;
    let t35649 = t171 * t5011;
    let t35651 = t31443 * t35649 * t35296;
    let t35653 = 0.13208198761633743869e-1 * t31479;
    let t35656 = t2030 * t8927 * t2297 * t1017;
    let t35660 = t7450 * t4262 * t2288 * t922;
    let t35662 = t7780 * t2310;
    (t35647, t35648, t35651, t35653, t35656, t35660, t35662)
}
