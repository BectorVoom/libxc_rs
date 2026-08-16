//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1010/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1010(t43490: f64, t7427: f64, t7573: f64, t43598: f64, t7572: f64, t10930: f64, t10931: f64, t43494: f64, t1445: f64, t2639: f64, t43307: f64, t44010: f64, t44012: f64, t44018: f64, t44020: f64, t44022: f64, t44024: f64, t44027: f64, t44029: f64, t44030: f64, t44033: f64, t44038: f64, t44040: f64, t44042: f64, t44046: f64, t44048: f64, t44051: f64, t833: f64) -> f64 {
    let t44053 = t7427 * t7573 * t43490;
    let t44057 = 0.62115540045351614476e2_f64 * t7572 * t7573 * t43598;
    let t44060 = 0.38649669361552115674e3_f64 * t10930 * t10931 * t43494;
    let t44061 = t44010 + 0.59584149919750711116e-1_f64 * t44012 + 0.11502877786176224903e2_f64 * t833 * t1445 * t43307 + 0.30674340763136599741e2_f64 * t44018 + 0.71500979903700853338e0_f64 * t44020 + 0.71500979903700853338e0_f64 * t44022 + 0.71500979903700853338e0_f64 * t44024 + t44027 + t44029 - 0.92023022289409799224e1_f64 * t44030 - 0.10725146985555128001e1_f64 * t44033 * t2639 - t44038 - t44040 - 0.12269736305254639897e2_f64 * t44042 + t44046 + 0.55213813373645879536e2_f64 * t44048 - t44051 - 0.12423108009070322895e3_f64 * t44053 + t44057 + t44060;
    t44061
}
