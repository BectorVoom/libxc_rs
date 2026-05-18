//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1215/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1215<F: Float>(t47886: F, t4741: F, t4756: F, t47938: F, t28489: F, t1303: F, t1317: F, t13504: F, t16221: F, t16292: F, t16579: F, t201: F, t22074: F, t3316: F, t3318: F, t37325: F, t37328: F, t37341: F, t4611: F, t4759: F, t714: F, t95: F) -> (F, F, F, F, F, F) {
    let t55944 = F::new(0.73246220147012639764e-3) * t47886;
    let t55945 = t4741 * t4741;
    let t55951 = t4756 * t4756;
    let t55977 = F::new(0.23392893589820816284e1) * t47938;
    let t55980 = F::new(0.22787712934626154593e-2) * t28489;
    let t55981 = F::new(2.0) * t3316 * t3318 * t16292 * t1317 * t201 + F::new(3.0) * t3316 * t3318 * t4756 * t4741 * t201 + F::new(0.62027715443768233192e-1) * t95 * t16221 * t1303 * t714 + F::new(3.0) * t4611 * t4759 + F::new(6.0) * t13504 * t16579 + F::new(70.0) / F::new(3.0) * t37325 - t55977 + F::new(6.0) * t37328 - F::new(14.0) * t37341 + t22074 - t55980;
    (t55944, t55945, t55951, t55977, t55980, t55981)
}
