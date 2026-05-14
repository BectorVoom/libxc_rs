//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1257/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1257<F: Float>(t10319: F, t4753: F, t2413: F, t26122: F, t26726: F, t901: F, t26822: F, t10315: F, t20445: F, t12963: F, t1540: F, t31347: F, t31358: F, t31361: F, t35120: F, t35123: F, t35126: F, t35128: F, t35130: F, t35133: F, t35136: F, t4130: F, t4781: F) -> (F,) {
    let t35138 = 0.47667319935800568892e0 * t10319 * t4753;
    let t35140 = 0.21450293971110256002e1 * t26122 * t2413;
    let t35141 = t26726 * t901;
    let t35142 = 0.29792074959875355558e-1 * t35141;
    let t35143 = t26822 * t901;
    let t35144 = 0.14896037479937677779e-1 * t35143;
    let t35146 = 0.14300195980740170668e1 * t20445 * t10315;
    let t35151 = -t35120 - t31347 - t35123 + t35126 + t35128 - t35130 - t35133 + t35136 - t35138 + t35140 + t35142 + t35144 - t35146 + 0.30674340763136599742e1 * t4781 * t4130 * t12963 * t1540 - t31358 - t31361;
    (t35151,)
}
