//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1014/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1014<F: Float>(t127: F, t645: F, t6856: F, t22166: F, t1928: F, t6926: F, t6931: F, t2030: F, t6938: F, t6936: F, t616: F, t6877: F, t6879: F) -> (F, F, F, F, F, F, F, F) {
    let t22168 = t6856 * t645 * t127;
    let t22169 = t22166 * t22168;
    let t22172 = t6926 * t1928;
    let t22173 = t6931 * t22172;
    let t22176 = t2030 * t6938;
    let t22178 = t6936 * t1928;
    let t22179 = t6931 * t22178;
    let t22187 = t6877 * t6879 * t616;
    (t22168, t22169, t22172, t22173, t22176, t22178, t22179, t22187)
}
