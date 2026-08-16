//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1065/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1065<F: Float>(t44005: F, t44010: F, t44027: F, t44029: F, t44038: F, t44040: F, t44046: F, t44051: F, t44057: F, t44060: F, t44064: F, t44069: F, t44074: F, t44079: F, t44083: F, t44085: F, t44089: F, t47486: F, t47488: F, t47492: F) -> F {
    let t51162 = t44005 + t44010 + t44027 + t44029 - t44038 - t44040 + t44046 - t44051 + t44057 + t44060 - t44064 + t44069 - t44074 + t44079 - t44083 - t44085 - t44089 - F::cast_from(0.15889106645266856298e0_f64) * t47486 - F::cast_from(0.29792074959875355558e-1_f64) * t47488 - F::cast_from(0.29792074959875355558e-1_f64) * t47492;
    t51162
}
