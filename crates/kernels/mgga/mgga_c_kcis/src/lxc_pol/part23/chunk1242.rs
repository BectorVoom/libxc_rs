//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1242/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1242<F: Float>(t1386: F, t16968: F, t27351: F, t27359: F, t28392: F, t5440: F, t7908: F, t94310: F, t94626: F, t98242: F, t98246: F, t98252: F, t98255: F, t98257: F, t98260: F, t98263: F, t98268: F) -> F {
    let t98270 = t16968 * t1386;
    let t98275 = -F::cast_from(0.92673611111111111113e-3_f64) * t94626 * t98242 + F::cast_from(0.27802083333333333334e-2_f64) * t7908 * t98246 - F::cast_from(0.12356481481481481481e-2_f64) * t28392 * t27359 + F::cast_from(0.99491666666666666664e-2_f64) * t98252 + t98255 + F::cast_from(0.22109259259259259258e-2_f64) * t98257 + F::cast_from(0.11054629629629629629e-2_f64) * t98260 - F::cast_from(0.44218518518518518517e-2_f64) * t98263 + F::cast_from(0.15445601851851851852e-3_f64) * t94310 - F::cast_from(0.73697530864197530861e-3_f64) * t98268 - F::cast_from(0.92673611111111111112e-3_f64) * t94626 * t98270 * t5440 * t27351;
    t98275
}
