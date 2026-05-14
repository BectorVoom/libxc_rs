//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 869/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk869<F: Float>(t44045: F, t10930: F, t10931: F, t43490: F, t24968: F, t9958: F, t7427: F, t7573: F, t43598: F, t7572: F, t43494: F, t1445: F, t2639: F, t43307: F, t44010: F, t44012: F, t44018: F, t44020: F, t44022: F, t44024: F, t44027: F, t44029: F, t44030: F, t44033: F, t44038: F, t44040: F, t44042: F, t833: F) -> (F,) {
    let t44046 = 0.63904876589867916128e-1 * t44045;
    let t44048 = t10930 * t10931 * t43490;
    let t44051 = 0.42900587942220512003e1 * t24968 * t9958;
    let t44053 = t7427 * t7573 * t43490;
    let t44057 = 0.62115540045351614476e2 * t7572 * t7573 * t43598;
    let t44060 = 0.38649669361552115674e3 * t10930 * t10931 * t43494;
    let t44061 = t44010 + 0.59584149919750711116e-1 * t44012 + 0.11502877786176224903e2 * t833 * t1445 * t43307 + 0.30674340763136599741e2 * t44018 + 0.71500979903700853338e0 * t44020 + 0.71500979903700853338e0 * t44022 + 0.71500979903700853338e0 * t44024 + t44027 + t44029 - 0.92023022289409799224e1 * t44030 - 0.10725146985555128001e1 * t44033 * t2639 - t44038 - t44040 - 0.12269736305254639897e2 * t44042 + t44046 + 0.55213813373645879536e2 * t44048 - t44051 - 0.12423108009070322895e3 * t44053 + t44057 + t44060;
    (t44061,)
}
