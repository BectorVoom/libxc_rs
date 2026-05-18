//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 701/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk701<F: Float>(t10003: F, t2379: F, t4985: F, t1707: F, t665: F, t903: F, t2024: F, t6522: F, t739: F, t236: F, t6108: F, t1971: F) -> (F, F, F, F, F, F, F) {
    let t10004 = F::new(0.59871208509319042821e-1) * t10003;
    let t10005 = t4985 * t2379;
    let t10006 = F::new(0.11974241701863808564e0) * t10005;
    let t10007 = t665 * t1707;
    let t10008 = t903 * t10007;
    let t10009 = F::new(0.35922725105591425692e0) * t10008;
    let t10010 = t2024 * t6522;
    let t10011 = t739 * t10010;
    let t10012 = F::new(0.23948483403727617128e0) * t10011;
    let t10013 = t236 * t6108;
    let t10014 = t1971 * t10013;
    (t10004, t10006, t10007, t10009, t10010, t10012, t10014)
}
