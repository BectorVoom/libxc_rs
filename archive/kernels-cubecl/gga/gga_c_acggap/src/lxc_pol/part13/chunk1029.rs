//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1029/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1029<F: Float>(t1181: F, t4822: F, t599: F, t8463: F, t30301: F, t34123: F, t34127: F, t34131: F, t34133: F, t34135: F, t34138: F, t34142: F, t34146: F, t34148: F, t34150: F, t34152: F, t34154: F, t34156: F, t34159: F, t34162: F) -> F {
    let t34166 = t8463 * t1181 * t599 * t4822;
    let t34169 = -F::cast_from(0.21437009059034868486e-3_f64) * t34123 + F::cast_from(0.14291339372689912324e-3_f64) * t34127 + t34131 - t34133 + F::cast_from(0.10718504529517434243e-2_f64) * t34135 + F::cast_from(0.42874018118069736972e-3_f64) * t34138 + F::cast_from(0.15724046144802076034e-2_f64) * t34142 + F::cast_from(0.31448092289604152068e-2_f64) * t34146 + F::cast_from(0.34299214494455789578e-2_f64) * t34148 - F::cast_from(0.17149607247227894789e-2_f64) * t34150 + F::cast_from(0.17149607247227894789e-2_f64) * t34152 - F::cast_from(0.85748036236139473944e-3_f64) * t34154 - F::cast_from(0.94344276868812456204e-3_f64) * t34156 - t34159 - F::cast_from(0.38586616306262763274e-1_f64) * t34162 + F::cast_from(0.32155513588552302729e-2_f64) * t34166 + F::cast_from(0.20007875121765877254e-2_f64) * t30301;
    t34169
}
