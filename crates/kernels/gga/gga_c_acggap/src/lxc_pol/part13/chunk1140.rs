//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1140/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1140<F: Float>(t31276: F, t8875: F, t1579: F, t2095: F, t355: F, t31477: F, t171: F, t5011: F, t31443: F, t35296: F, t31479: F, t1017: F, t2030: F, t2297: F, t8927: F) -> (F, F, F, F, F, F) {
    let t35643 = t31276 * t8875;
    let t35646 = t2095 * t1579 * t355;
    let t35647 = F::new(0.1528125e-1) * t35646;
    let t35648 = F::new(0.13073958333333333333e0) * t31477;
    let t35649 = t171 * t5011;
    let t35651 = t31443 * t35649 * t35296;
    let t35653 = F::new(0.13208198761633743869e-1) * t31479;
    let t35656 = t2030 * t8927 * t2297 * t1017;
    (t35643, t35647, t35648, t35651, t35653, t35656)
}
