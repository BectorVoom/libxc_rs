//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 773/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk773<F: Float>(t265: F, t794: F, t262: F, t7844: F, t321: F, t7617: F, t5271: F, t2079: F, t352: F, t830: F, t333: F, t2073: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35888 = t265 * t794;
    let t35889 = t262 * t35888;
    let t35890 = t7844 * t35889;
    let t35917 = t7617 * t321;
    let t35918 = t5271 * t35917;
    let t35922 = t2079 * t262 * t830 * t352;
    let t35924 = t830 * t333;
    let t35925 = t262 * t35924;
    let t35926 = t2073 * t35925;
    (t35888, t35889, t35890, t35917, t35918, t35922, t35924, t35925, t35926)
}
