//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 630/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk630<F: Float>(t1734: F, t649: F, t27: F, t2134: F, t2333: F, t8368: F, t1763: F, t7577: F, t739: F, t2289: F, t2412: F, t1942: F, t1986: F, t675: F, t2310: F, t1835: F, t202: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9806 = t649 * t1734;
    let t9807 = t27 * t9806;
    let t9808 = t2134 * t9807;
    let t9810 = t8368 * t2333;
    let t9812 = t7577 * t1763;
    let t9813 = t739 * t9812;
    let t9815 = t2412 * t2289;
    let t9817 = t1986 * t1942;
    let t9818 = t675 * t9817;
    let t9820 = t2412 * t2310;
    let t9824 = t1835 * t202;
    (t9807, t9808, t9810, t9812, t9813, t9815, t9817, t9818, t9820, t9824)
}
