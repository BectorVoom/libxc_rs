//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 352/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk352<F: Float>(t325: F, t504: F, t1328: F, t1003: F, t230: F, t1171: F, t225: F, t226: F, t463: F, t892: F, t337: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4041 = t504 * t325;
    let t4071 = F::cast_from(1.0_f64) / t1328;
    let t4179 = F::cast_from(1.0_f64) / t1003 / t230;
    let t4441 = t1171 * t225;
    let t4443 = F::cast_from(1.0_f64) / t226 / t4441;
    let t4516 = t463 * t463;
    let t4517 = F::cast_from(1.0_f64) / t4516;
    let t4601 = t892 * t325;
    let t4615 = t337 * t337;
    let t4616 = F::cast_from(1.0_f64) / t4615;
    (t4041, t4071, t4179, t4441, t4443, t4517, t4601, t4615, t4616)
}
