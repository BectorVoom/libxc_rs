//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1222/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1222<F: Float>(t31537: F, t7467: F, t120112: F, t114418: F, t1983: F, t7687: F, t15868: F, t8489: F, t22751: F, t32731: F, t22633: F, t22635: F, t31099: F, t5187: F) -> (F, F, F, F, F, F) {
    let t120140 = F::cast_from(4.0_f64) * t31537 * t7467;
    let t120165 = F::cast_from(2.0_f64) * t120112;
    let t120171 = F::cast_from(3.0_f64) * t1983 * t114418 * t7687;
    let t120176 = t1983 * t8489 * t15868;
    let t120179 = t22751 * t32731;
    let t120180 = F::cast_from(0.76763589786250567037e-1_f64) * t120179;
    let t120184 = F::cast_from(0.3289868133696452873e-1_f64) * t22633 * t22635 * t31099 * t5187;
    (t120140, t120165, t120171, t120176, t120180, t120184)
}
