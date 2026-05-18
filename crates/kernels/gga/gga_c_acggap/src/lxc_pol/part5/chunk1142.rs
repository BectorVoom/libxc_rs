//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1142/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1142<F: Float>(t1008: F, t5946: F, t1032: F, t6081: F, t1856: F, t3670: F, t1036: F, t1095: F, t1795: F, t398: F, t864: F, t1017: F, t1131: F, t12615: F, t12621: F, t12623: F, t12626: F, t12641: F, t12646: F, t1426: F, t15529: F, t1713: F, t1772: F, t3300: F, t418: F) -> F {
    let t20471 = t1008 * t5946;
    let t20478 = t1032 * t6081;
    let t20480 = t3670 * t1856;
    let t20490 = t1036 * t398 * t1095 * t1795 * t864;
    let t20495 = F::new(0.48018900292238105409e-1) * t12615 - F::new(0.24009450146119052705e-1) * t12621 + F::new(0.24009450146119052705e-1) * t12623 - t12626 - F::new(0.17149607247227894789e-1) * t20471 - F::new(0.85748036236139473944e-2) * t418 * t1426 * t1095 * t1713 * t1131 - F::new(0.40015750243531754508e-2) * t20478 - F::new(0.45351183609335988442e-1) * t20480 + F::new(0.25724410870841842183e-2) * t418 * t398 * t3300 * t1772 * t1017 - F::new(0.85748036236139473944e-3) * t20490 - F::new(0.17149607247227894789e-2) * t15529 - F::new(455.0) / F::new(648.0) * t12641 + F::new(35.0) / F::new(432.0) * t12646;
    t20495
}
