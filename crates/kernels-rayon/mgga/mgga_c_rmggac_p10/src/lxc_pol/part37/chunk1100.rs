//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1100/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1100(t76161: f64, t76163: f64, t76165: f64, t76167: f64, t76169: f64, t77845: f64, t77846: f64, t77848: f64, t77849: f64, t77850: f64, t77851: f64, t77852: f64, t77853: f64) -> f64 {
    let t80413 = -t77845 + t77846 - t77848 + t77849 + t77850 + t77851 + t77852 - t77853 - 0.18637685463734316848e-1_f64 * t76161 + 0.46594213659335792121e-1_f64 * t76163 + 0.93188427318671584242e-2_f64 * t76165 + 0.46594213659335792121e-1_f64 * t76167 - 0.93188427318671584242e-1_f64 * t76169;
    t80413
}
