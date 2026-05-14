//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 984/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk984<F: Float>(t5351: F, t5371: F, t1220: F, t1914: F, t863: F, t864: F, t316: F, t322: F, t449: F, t6557: F, t1907: F, t862: F, t865: F, t1659: F, t4137: F, t5517: F, t852: F) -> (F, F, F, F, F, F) {
    let t19607 = t5371 * t5351;
    let t19611 = t863 * t1220 * t1914 * t864;
    let t19615 = t316 * t449 * t6557 * t322;
    let t19618 = t862 * t1907 * t865;
    let t19620 = t4137 * t1659;
    let t19627 = t852 * t5517;
    (t19607, t19611, t19615, t19618, t19620, t19627)
}
