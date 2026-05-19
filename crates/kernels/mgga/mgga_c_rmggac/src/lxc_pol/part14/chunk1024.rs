//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1024/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1024<F: Float>(t41377: F, t2103: F, t41036: F, t2118: F, t36175: F, t36184: F, t36192: F, t36194: F, t36201: F, t36205: F, t41363: F, t41365: F, t41367: F, t41368: F, t41371: F, t41373: F, t41375: F) -> F {
    let t41378 = F::cast_from(0.18183107769496894486e-1_f64) * t41377;
    let t41379 = t2103 * t41036;
    let t41380 = F::cast_from(0.24244143692662525982e-1_f64) * t41379;
    let t41381 = t2118 * t41036;
    let t41383 = -F::cast_from(0.10620923284048465071e-2_f64) * t36175 + F::cast_from(0.3540307761349488357e-2_f64) * t36184 + F::cast_from(0.33335697577410973225e-1_f64) * t41363 + F::cast_from(0.88704377798256624947e-3_f64) * t41365 - t41367 + t41368 + F::cast_from(0.74346462988339255497e-2_f64) * t36192 + F::cast_from(0.88507694033737208925e-3_f64) * t36194 + t36201 + F::cast_from(0.53218852008283593618e-1_f64) * t41371 + F::cast_from(0.53218852008283593618e-1_f64) * t41373 - t36205 - F::cast_from(0.10584045078201074568e-3_f64) * t41375 - t41378 + t41380 + F::cast_from(0.56448240417072397696e-3_f64) * t41381;
    t41383
}
