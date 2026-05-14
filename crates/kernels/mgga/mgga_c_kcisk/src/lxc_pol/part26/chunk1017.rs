//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1017/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1017<F: Float>(t27145: F, t498: F, t493: F, t1517: F, t8260: F, t26750: F, t470: F, t487: F, t1487: F, t1483: F, t8256: F, t1492: F, t8251: F, t486: F, t26416: F, t382: F) -> (F, F, F, F, F, F, F) {
    let t27146 = t498 * t27145;
    let t27147 = t493 * t27146;
    let t27149 = t8260 * t1517;
    let t27151 = t470 * t26750;
    let t27152 = t487 * t27151;
    let t27153 = t1487 * t27152;
    let t27155 = t1483 * t8256;
    let t27157 = t1492 * t8251;
    let t27158 = t486 * t27157;
    let t27160 = t382 * t26416;
    (t27146, t27147, t27149, t27153, t27155, t27158, t27160)
}
