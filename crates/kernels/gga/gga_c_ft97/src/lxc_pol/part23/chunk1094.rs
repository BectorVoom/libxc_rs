//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1094/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1094<F: Float>(t1403: F, t1426: F, t9555: F, t24211: F, t5996: F, t24191: F, t683: F, t2399: F, t6010: F, t6061: F, t7514: F, t10051: F, t1443: F, t458: F, t6108: F) -> (F, F, F, F, F, F, F) {
    let t96770 = 14.0 / 81.0 * t1403 * t9555 * t1426;
    let t96782 = t5996 * t24211;
    let t96798 = t683 * t24191;
    let t96818 = t1403 * t2399 * t6010;
    let t96824 = t7514 * t6061;
    let t96834 = t1443 * t10051;
    let t96925 = t6108 * t458;
    (t96770, t96782, t96798, t96818, t96824, t96834, t96925)
}
