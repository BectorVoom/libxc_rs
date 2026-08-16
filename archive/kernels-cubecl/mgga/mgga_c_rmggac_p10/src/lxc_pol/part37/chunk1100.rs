//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1100/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1100<F: Float>(t76161: F, t76163: F, t76165: F, t76167: F, t76169: F, t77845: F, t77846: F, t77848: F, t77849: F, t77850: F, t77851: F, t77852: F, t77853: F) -> F {
    let t80413 = -t77845 + t77846 - t77848 + t77849 + t77850 + t77851 + t77852 - t77853 - F::cast_from(0.18637685463734316848e-1_f64) * t76161 + F::cast_from(0.46594213659335792121e-1_f64) * t76163 + F::cast_from(0.93188427318671584242e-2_f64) * t76165 + F::cast_from(0.46594213659335792121e-1_f64) * t76167 - F::cast_from(0.93188427318671584242e-1_f64) * t76169;
    t80413
}
