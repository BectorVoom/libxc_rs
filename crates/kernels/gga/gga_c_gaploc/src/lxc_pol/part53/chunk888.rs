//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 888/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk888<F: Float>(t10736: F, t29277: F, t7064: F, t10635: F, t2554: F, t1841: F, t3487: F, t734: F, t9641: F, t40588: F, t40602: F, t13194: F, t29439: F) -> (F, F, F, F, F, F) {
    let t42970 = t7064 * t29277 * t10736;
    let t42973 = t7064 * t10635 * t2554;
    let t42974 = F::cast_from(0.64087718584518535698e-3_f64) * t42973;
    let t42978 = F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t9641 * t3487 * t734;
    let t42980 = F::cast_from(0.1922631557535556071e-2_f64) * t40588;
    let t42984 = F::cast_from(0.1281754371690370714e-2_f64) * t40602;
    let t42985 = t29439 * t13194;
    (t42970, t42974, t42978, t42980, t42984, t42985)
}
