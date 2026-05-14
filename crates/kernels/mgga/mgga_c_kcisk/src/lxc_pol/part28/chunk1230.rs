//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1230/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1230<F: Float>(t34676: F, t564: F, t2053: F, t2359: F, t2776: F, t2707: F, t7724: F, t566: F, t9295: F, t2820: F, t8464: F, t34093: F, t9945: F, t1799: F, t6666: F, t7715: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34677 = t564 * t34676;
    let t34679 = t2359 * t2053;
    let t34680 = t2776 * t34679;
    let t35059 = t7724 * t2707;
    let t35076 = t566 * t9295;
    let t35077 = t2776 * t35076;
    let t35078 = t35077 / 16.0;
    let t35079 = t8464 * t2820;
    let t35080 = t35079 / 8.0;
    let t35081 = t34093 * t9945;
    let t35082 = t1799 * t35081;
    let t35084 = t6666 * t7715;
    (t34677, t34679, t34680, t35059, t35078, t35080, t35081, t35082, t35084)
}
