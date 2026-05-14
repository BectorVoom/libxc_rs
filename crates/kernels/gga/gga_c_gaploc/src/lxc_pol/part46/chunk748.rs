//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 748/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk748<F: Float>(t40280: F, t40283: F, t1445: F, t1562: F, t41784: F, t12881: F, t9497: F, t8248: F, t9565: F, t40301: F, t41809: F, t6508: F, t4820: F, t6824: F, t40245: F, t41968: F, t41970: F, t41972: F, t41973: F, t41974: F, t41975: F, t41976: F, t41978: F, t41979: F, t41980: F, t41981: F, t41982: F) -> (F, F) {
    let t41983 = 0.11916829983950142223e0 * t40280;
    let t41984 = 0.59584149919750711116e-1 * t40283;
    let t41987 = 0.62115540045351614476e2 * t1562 * t1445 * t41784;
    let t41989 = 0.25025342966295298669e1 * t9497 * t12881;
    let t41991 = 0.11916829983950142223e0 * t8248 * t9565;
    let t41992 = 0.38342925953920749676e1 * t40301;
    let t41993 = t6508 * t41809;
    let t41996 = 0.79445533226334281487e-1 * t6824 * t4820 * t41993;
    let t41997 = t41968 + 0.92023022289409799224e1 * t41970 - t41972 - t41973 + t41974 - t41975 - t41976 - 0.76685851907841499352e0 * t40245 + t41978 + t41979 - t41980 - t41981 + t41982 - t41983 + t41984 - t41987 - t41989 + t41991 + t41992 - t41996;
    (t41993, t41997)
}
