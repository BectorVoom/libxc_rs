//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 855/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk855(t40301: f64, t41809: f64, t6508: f64, t4820: f64, t6824: f64, t40245: f64, t41968: f64, t41970: f64, t41972: f64, t41973: f64, t41974: f64, t41975: f64, t41976: f64, t41978: f64, t41979: f64, t41980: f64, t41981: f64, t41982: f64, t41983: f64, t41984: f64, t41987: f64, t41989: f64, t41991: f64) -> (f64, f64) {
    let t41992 = 0.38342925953920749676e1_f64 * t40301;
    let t41993 = t6508 * t41809;
    let t41996 = 0.79445533226334281487e-1_f64 * t6824 * t4820 * t41993;
    let t41997 = t41968 + 0.92023022289409799224e1_f64 * t41970 - t41972 - t41973 + t41974 - t41975 - t41976 - 0.76685851907841499352e0_f64 * t40245 + t41978 + t41979 - t41980 - t41981 + t41982 - t41983 + t41984 - t41987 - t41989 + t41991 + t41992 - t41996;
    (t41993, t41997)
}
