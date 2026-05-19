//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 703/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk703<F: Float>(t12701: F, t12706: F, t10628: F, t2365: F, t6111: F, t12709: F, t2949: F, t3234: F, t1445: F, t813: F, t13023: F, t833: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13116 = F::cast_from(0.29792074959875355558e-1_f64) * t12701;
    let t13117 = F::cast_from(0.63904876589867916127e-1_f64) * t12706;
    let t13118 = t2365 * t10628;
    let t13119 = t6111 * t13118;
    let t13120 = F::cast_from(0.59584149919750711116e-1_f64) * t13119;
    let t13124 = F::cast_from(0.19171462976960374838e1_f64) * t12709;
    let t13129 = t2949 * t3234;
    let t13130 = t1445 * t13129;
    let t13132 = F::cast_from(0.46011511144704899612e1_f64) * t813 * t13130;
    let t13136 = t1445 * t13023;
    let t13138 = F::cast_from(0.11502877786176224903e2_f64) * t833 * t13136;
    (t13116, t13117, t13118, t13120, t13124, t13129, t13130, t13132, t13136, t13138)
}
