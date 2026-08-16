//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1322/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1322(t11371: f64, t2099: f64, t918: f64, t11338: f64, t927: f64, t11341: f64, t3174: f64, t68: f64, t10197: f64, t3192: f64, t10106: f64, t10236: f64, t11445: f64, t1238: f64, t28059: f64, t28061: f64, t2888: f64, t3026: f64, t31948: f64, t31989: f64, t3242: f64, t3860: f64, t405: f64, t6404: f64, t758: f64, t824: f64, t921: f64) -> f64 {
    let t31996 = t918 * t2099 * t11371;
    let t32007 = t11338 * t927;
    let t32010 = t3174 * t68 * t11341;
    let t32014 = t10197 * t3192;
    let t32016 = -0.43445671692977333464e-1_f64 * t3860 * t3242 + 0.68598428988911579154e-2_f64 * t1238 * t10236 - 0.28963781128651555643e-1_f64 * t31948 + 0.21437009059034868486e-3_f64 * t918 * t758 * t405 * t31989 * t921 + 0.14291339372689912324e-3_f64 * t31996 + t3174 * t2888 * t6404 * t11445 * t824 / 4.0_f64 - 3.0_f64 / 16.0_f64 * t3174 * t2888 * t10106 * t3026 - 0.35400176935018568008e-1_f64 * t32007 + t32010 / 48.0_f64 - 0.85748036236139473944e-3_f64 * t28059 + 0.91464571985215438873e-2_f64 * t28061 + 0.14481890564325777821e-1_f64 * t32014;
    t32016
}
