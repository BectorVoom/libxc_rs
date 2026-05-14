//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 945/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk945<F: Float>(t1140: F, t5171: F, t1315: F, t13787: F, t1328: F, t3573: F, t12930: F, t1466: F, t3409: F, t4681: F, t4685: F, t4331: F, t14173: F, t4425: F, t12816: F, t13298: F, t13299: F, t525: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18085 = t1140 * t5171;
    let t18087 = t13787 * t1315;
    let t18097 = t3573 * t1328;
    let t18103 = t12930 * t1466;
    let t18105 = t3409 * t4681;
    let t18107 = t3409 * t4685;
    let t18109 = t3409 * t4331;
    let t18111 = t14173 * t4425;
    let t18119 = t13298 * t13299 * t525 * t12816;
    (t18085, t18087, t18097, t18103, t18105, t18107, t18109, t18111, t18119)
}
