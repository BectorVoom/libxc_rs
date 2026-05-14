//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1375/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1375<F: Float>(t3837: F, t6117: F, t8145: F, t8219: F, t2198: F, t3740: F, t6142: F, t2295: F, t27453: F, t890: F, t898: F, t10168: F, t18580: F, t2235: F, t9976: F, t2239: F, t3734: F) -> (F, F, F, F, F, F, F) {
    let t27479 = 0.5848223622634646207e0 * t6117 * t3837;
    let t27481 = 12.0 * t8219 * t8145;
    let t27484 = 24.0 * t6142 * t3740 * t2198;
    let t27488 = 0.23392894490538584828e1 * t898 * t2295 * t27453 * t890;
    let t27491 = 0.10254018858216406658e4 * t898 * t10168 * t18580;
    let t27493 = 1.0 * t9976 * t2235;
    let t27494 = t3734 * t2239;
    (t27479, t27481, t27484, t27488, t27491, t27493, t27494)
}
