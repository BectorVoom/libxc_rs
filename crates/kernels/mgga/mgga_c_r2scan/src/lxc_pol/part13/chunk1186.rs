//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1186/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1186<F: Float>(t37982: F, t7620: F, t10856: F, t7407: F, t38153: F, t10868: F, t2147: F, t8066: F, t7470: F, t38144: F, t38147: F, t38150: F, t38156: F, t38158: F, t38161: F, t38165: F) -> F {
    let t40232 = t37982 * t7620;
    let t40233 = F::new(0.19514881078765566037e-1) * t40232;
    let t40234 = t10856 * t7407;
    let t40238 = F::new(0.57829097596741960692e-3) * t38153;
    let t40241 = t2147 * t10868 * t8066;
    let t40242 = F::new(0.46574606203128791246e-1) * t40241;
    let t40243 = t10856 * t7470;
    let t40244 = F::new(0.19514881078765566037e-1) * t40243;
    let t40247 = t40233 + F::new(0.58544643236296698113e-1) * t40234 - t38144 + F::new(0.46574606203128791246e-1) * t38147 + F::new(0.16262400898971305032e-3) * t38150 - t40238 + F::new(0.13972381860938637374e0) * t38156 - t40242 - t40244 + F::new(0.64025200389650807209e-1) * t38158 - F::new(0.46574606203128791246e-1) * t38161 + t38165;
    t40247
}
