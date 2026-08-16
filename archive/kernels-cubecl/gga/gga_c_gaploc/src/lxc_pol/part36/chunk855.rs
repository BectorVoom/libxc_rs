//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 855/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk855<F: Float>(t40301: F, t41809: F, t6508: F, t4820: F, t6824: F, t40245: F, t41968: F, t41970: F, t41972: F, t41973: F, t41974: F, t41975: F, t41976: F, t41978: F, t41979: F, t41980: F, t41981: F, t41982: F, t41983: F, t41984: F, t41987: F, t41989: F, t41991: F) -> (F, F) {
    let t41992 = F::cast_from(0.38342925953920749676e1_f64) * t40301;
    let t41993 = t6508 * t41809;
    let t41996 = F::cast_from(0.79445533226334281487e-1_f64) * t6824 * t4820 * t41993;
    let t41997 = t41968 + F::cast_from(0.92023022289409799224e1_f64) * t41970 - t41972 - t41973 + t41974 - t41975 - t41976 - F::cast_from(0.76685851907841499352e0_f64) * t40245 + t41978 + t41979 - t41980 - t41981 + t41982 - t41983 + t41984 - t41987 - t41989 + t41991 + t41992 - t41996;
    (t41993, t41997)
}
