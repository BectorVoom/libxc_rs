//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1242/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1242(t1386: f64, t16968: f64, t27351: f64, t27359: f64, t28392: f64, t5440: f64, t7908: f64, t94310: f64, t94626: f64, t98242: f64, t98246: f64, t98252: f64, t98255: f64, t98257: f64, t98260: f64, t98263: f64, t98268: f64) -> f64 {
    let t98270 = t16968 * t1386;
    let t98275 = -0.92673611111111111113e-3_f64 * t94626 * t98242 + 0.27802083333333333334e-2_f64 * t7908 * t98246 - 0.12356481481481481481e-2_f64 * t28392 * t27359 + 0.99491666666666666664e-2_f64 * t98252 + t98255 + 0.22109259259259259258e-2_f64 * t98257 + 0.11054629629629629629e-2_f64 * t98260 - 0.44218518518518518517e-2_f64 * t98263 + 0.15445601851851851852e-3_f64 * t94310 - 0.73697530864197530861e-3_f64 * t98268 - 0.92673611111111111112e-3_f64 * t94626 * t98270 * t5440 * t27351;
    t98275
}
