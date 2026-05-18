//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1039/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1039<F: Float>(t1767: F, t9476: F, t1096: F, t1092: F, t341: F, t9368: F, t1017: F, t86: F, t359: F, t9372: F, t1646: F, t2630: F) -> (F, F, F, F) {
    let t13124 = t9476 * t1767;
    let t13125 = t1096 * t13124;
    let t13126 = t1092 * t13125;
    let t13128 = t9368 * t341;
    let t13130 = t86 * t1017 * t13128;
    let t13131 = t359 * t9372;
    let t13132 = t1646 * t2630;
    (t13126, t13130, t13131, t13132)
}
