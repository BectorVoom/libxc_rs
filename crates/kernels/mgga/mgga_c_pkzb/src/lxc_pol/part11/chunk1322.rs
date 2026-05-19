//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1322/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1322<F: Float>(t11371: F, t2099: F, t918: F, t11338: F, t927: F, t11341: F, t3174: F, t68: F, t10197: F, t3192: F, t10106: F, t10236: F, t11445: F, t1238: F, t28059: F, t28061: F, t2888: F, t3026: F, t31948: F, t31989: F, t3242: F, t3860: F, t405: F, t6404: F, t758: F, t824: F, t921: F) -> F {
    let t31996 = t918 * t2099 * t11371;
    let t32007 = t11338 * t927;
    let t32010 = t3174 * t68 * t11341;
    let t32014 = t10197 * t3192;
    let t32016 = -F::cast_from(0.43445671692977333464e-1_f64) * t3860 * t3242 + F::cast_from(0.68598428988911579154e-2_f64) * t1238 * t10236 - F::cast_from(0.28963781128651555643e-1_f64) * t31948 + F::cast_from(0.21437009059034868486e-3_f64) * t918 * t758 * t405 * t31989 * t921 + F::cast_from(0.14291339372689912324e-3_f64) * t31996 + t3174 * t2888 * t6404 * t11445 * t824 / F::new(4.0) - F::new(3.0) / F::new(16.0) * t3174 * t2888 * t10106 * t3026 - F::cast_from(0.35400176935018568008e-1_f64) * t32007 + t32010 / F::new(48.0) - F::cast_from(0.85748036236139473944e-3_f64) * t28059 + F::cast_from(0.91464571985215438873e-2_f64) * t28061 + F::cast_from(0.14481890564325777821e-1_f64) * t32014;
    t32016
}
