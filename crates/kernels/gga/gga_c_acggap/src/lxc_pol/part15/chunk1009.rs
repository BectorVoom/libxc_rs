//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1009/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1009<F: Float>(t2016: F, t9618: F, t1488: F, t2030: F, t2313: F, t2001: F, t5551: F, t1856: F, t7605: F, t5811: F, t5816: F, t1988: F, t9577: F, t1095: F, t1980: F, t30058: F, t5655: F) -> (F, F, F, F, F, F, F, F) {
    let t39925 = t2016 * t9618;
    let t39928 = t2030 * t1488 * t2313;
    let t39930 = t2001 * t5551;
    let t39932 = t7605 * t1856;
    let t39934 = t2001 * t5811;
    let t39937 = t2001 * t5816;
    let t39939 = t1988 * t9577;
    let t39944 = t1980 * t30058 * t1095 * t5655;
    (t39925, t39928, t39930, t39932, t39934, t39937, t39939, t39944)
}
