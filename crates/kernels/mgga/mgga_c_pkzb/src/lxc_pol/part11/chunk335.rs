//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 335/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk335<F: Float>(t1169: F, t1175: F, t1178: F, t1182: F, t865: F, t868: F) -> F {
    let t1196 = F::cast_from(0.3529725e1_f64) * t1175 - t865 + F::cast_from(0.1549425e1_f64) * t1169 + F::cast_from(0.6311625e0_f64) * t1178 - t868 + F::cast_from(0.312585e0_f64) * t1182;
    t1196
}
