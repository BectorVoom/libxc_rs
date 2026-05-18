//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1425/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1425<F: Float>(t35141: F, t26822: F, t901: F, t10315: F, t20445: F, t12963: F, t1540: F, t31347: F, t31358: F, t31361: F, t35120: F, t35123: F, t35126: F, t35128: F, t35130: F, t35133: F, t35136: F, t35138: F, t35140: F, t4130: F, t4781: F) -> F {
    let t35142 = F::new(0.29792074959875355558e-1) * t35141;
    let t35143 = t26822 * t901;
    let t35144 = F::new(0.14896037479937677779e-1) * t35143;
    let t35146 = F::new(0.14300195980740170668e1) * t20445 * t10315;
    let t35151 = -t35120 - t31347 - t35123 + t35126 + t35128 - t35130 - t35133 + t35136 - t35138 + t35140 + t35142 + t35144 - t35146 + F::new(0.30674340763136599742e1) * t4781 * t4130 * t12963 * t1540 - t31358 - t31361;
    t35151
}
