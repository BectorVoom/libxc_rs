//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1004/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1004<F: Float>(t7979: F, t7982: F, t6177: F, t6218: F, t7970: F, t7973: F, t7975: F, t7986: F, t7990: F, t7994: F, t7997: F, t8000: F) -> F {
    let t8090 = F::cast_from(0.33114e0_f64) * t7979;
    let t8091 = F::cast_from(0.33114e0_f64) * t7982;
    let t8097 = -F::cast_from(0.1294625e1_f64) * t7970 + F::cast_from(0.16504875e0_f64) * t7973 + F::cast_from(0.82524375e-1_f64) * t7975 - t6218 + F::cast_from(0.5519e0_f64) * t6177 - t8090 - t8091 + F::cast_from(0.248355e0_f64) * t7986 + F::cast_from(0.49671e0_f64) * t7990 + F::cast_from(0.248355e0_f64) * t7994 + F::cast_from(0.19419375e1_f64) * t7997 - F::cast_from(0.412621875e-1_f64) * t8000;
    t8097
}
