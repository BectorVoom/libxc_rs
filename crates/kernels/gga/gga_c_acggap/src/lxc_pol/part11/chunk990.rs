//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 990/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk990<F: Float>(t2297: F, t3176: F, t13287: F, t31195: F, t1980: F, t34487: F, t7476: F, t2118: F, t5082: F, t31126: F, t31128: F, t142: F, t2060: F, t4838: F, t604: F, t2314: F, t31258: F) -> (F, F, F, F, F, F, F, F) {
    let t35340 = t2297 * t3176;
    let t35342 = t31195 * t13287 * t35340;
    let t35348 = t1980 * t7476 * t34487;
    let t35349 = 0.7145669686344956162e-3 * t35348;
    let t35350 = t2118 * t5082;
    let t35352 = 0.1324375e0 * t31126;
    let t35353 = 0.57165357490759649296e-3 * t31128;
    let t35357 = t2060 * t142 * t604 * t4838;
    let t35359 = t31258 * t2314;
    (t35340, t35342, t35349, t35350, t35352, t35353, t35357, t35359)
}
