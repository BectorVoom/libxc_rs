//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1218/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1218<F: Float>(t1096: F, t17388: F, t17616: F, t21220: F, t21223: F, t21225: F, t21226: F, t21229: F, t21233: F, t21236: F, t21239: F, t21251: F, t21255: F, t21257: F, t2801: F, t2820: F, t5830: F, t5831: F, t5883: F, t704: F, t723: F, t7486: F) -> F {
    let t21258 = -t21220 - t21223 - t21225 + F::cast_from(0.17544670867903938621e1_f64) * t21226 * t723 + F::cast_from(3.0_f64) * t21229 * t704 - t21233 - t21236 - t21239 - F::cast_from(24.0_f64) * t5830 * t1096 * t5831 - F::cast_from(6.0_f64) * t7486 * t5883 - F::cast_from(6.0_f64) * t17388 * t2801 + F::cast_from(0.96491876992155210402e2_f64) * t17616 * t2820 + t21251 - t21255 - t21257;
    t21258
}
