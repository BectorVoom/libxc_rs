//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1268/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1268<F: Float>(t33162: F, t9721: F, t9733: F, t2804: F, t33262: F, t4419: F, t5439: F, t5507: F, t17182: F, t33199: F, t9740: F, t18681: F, t2806: F, t33196: F, t33291: F, t9736: F) -> (F, F, F, F, F, F, F, F) {
    let t112960 = t9721 * t33162;
    let t112962 = t9733 * t33162;
    let t112975 = t2804 * t4419 * t33262;
    let t112982 = t5507 * t5439;
    let t112988 = t17182 * t33199;
    let t112989 = t9740 * t112988;
    let t113003 = 0.19290123456790123457e-2 * t2804 * t18681 * t2806;
    let t113009 = t33196 * t112988;
    let t113022 = t33291 * t9736;
    (t112960, t112962, t112975, t112982, t112989, t113003, t113009, t113022)
}
