//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 641/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk641<F: Float>(t7418: F, t9931: F, t675: F, t1927: F, t1986: F, t1937: F, t1707: F, t645: F, t3928: F, t2060: F, t6522: F, t1550: F, t1756: F, t739: F, t515: F, t3352: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9932 = t7418 * t9931;
    let t9933 = t675 * t9932;
    let t9935 = t1986 * t1927;
    let t9936 = t675 * t9935;
    let t9938 = t1986 * t1937;
    let t9939 = t675 * t9938;
    let t9948 = t645 * t1707;
    let t9949 = t3928 * t9948;
    let t9951 = t2060 * t6522;
    let t9952 = t1550 * t9951;
    let t9957 = t2060 * t1756;
    let t9958 = t739 * t9957;
    let t9963 = t515 * t6522;
    let t9964 = t3352 * t9963;
    (t9932, t9933, t9935, t9936, t9938, t9939, t9948, t9949, t9951, t9952, t9957, t9958, t9964)
}
