//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 969/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk969<F: Float>(t14668: F, t7743: F, t5036: F, t7766: F, t10491: F, t8064: F, t1203: F, t10498: F, t1820: F, t3330: F, t2189: F, t5189: F, t3325: F, t8081: F, t1176: F, t1796: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t27995 = 2.0 * t14668 * t7743;
    let t27996 = t5036 * t7766;
    let t27998 = 2.0 * t10491 * t8064;
    let t27999 = t8064 * t1203;
    let t28001 = 6.0 * t10498 * t27999;
    let t28002 = t7766 * t1820;
    let t28004 = 2.0 * t3330 * t28002;
    let t28005 = t2189 * t5189;
    let t28007 = 2.0 * t3330 * t28005;
    let t28008 = t3325 * t8081;
    let t28009 = t8081 * t1203;
    let t28011 = 2.0 * t3330 * t28009;
    let t28012 = t1796 * t1176;
    (t27995, t27996, t27998, t27999, t28001, t28002, t28004, t28005, t28007, t28008, t28009, t28011, t28012)
}
