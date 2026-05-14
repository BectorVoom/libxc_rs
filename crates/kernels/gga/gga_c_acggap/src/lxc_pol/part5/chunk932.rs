//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 932/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk932<F: Float>(t12727: F, t1466: F, t3382: F, t4681: F, t3706: F, t524: F, t13083: F, t2450: F, t4465: F, t14056: F, t4732: F, t4396: F, t4456: F, t157: F, t3101: F, t1163: F, t1165: F, t1532: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17540 = t12727 * t1466;
    let t17542 = t3382 * t4681;
    let t17544 = t3706 * t524;
    let t17550 = t2450 * t13083;
    let t17551 = t17550 * t4465;
    let t17557 = t14056 * t4732;
    let t17567 = t4396 * t4456;
    let t17581 = t157 * t3101;
    let t17584 = t1163 * t1165 * t1532 * t17581;
    (t17540, t17542, t17544, t17550, t17551, t17557, t17567, t17581, t17584)
}
