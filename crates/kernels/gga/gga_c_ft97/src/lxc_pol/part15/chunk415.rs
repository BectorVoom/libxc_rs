//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 415/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk415<F: Float>(t1170: F, t1882: F, t1144: F, t1186: F, t2336: F, t89: F, t1213: F, t375: F, t1212: F, t2680: F, t1196: F, t816: F, t1095: F, t2697: F, t280: F, t283: F) -> (F, F, F, F, F, F, F, F) {
    let t3986 = t1882 * t1170;
    let t3988 = t1882 * t1144;
    let t4032 = t89 * t2336 * t1186;
    let t4049 = t89 * t375 * t1213;
    let t4056 = t2680 * t1212;
    let t4064 = t816 * t1196;
    let t4068 = t2697 * t1095;
    let t4092 = t280 * t283;
    (t3986, t3988, t4032, t4049, t4056, t4064, t4068, t4092)
}
