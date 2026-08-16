//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2685/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2685<F: Float>(t3923: F, t48105: F, t2782: F, t47371: F, t1399: F, t14122: F, t4057: F, t47424: F, t47427: F, t47432: F, t47436: F, t49205: F, t49378: F, t49382: F, t49386: F, t5735: F, t5755: F, t9891: F) -> F {
    let t49393 = t48105 * t3923;
    let t49395 = t2782 * t47371 * t49393;
    let t49397 = -F::cast_from(0.39512695097613069591e1_f64) * t5755 * t49205 * t1399 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t14122 * t4057 + F::cast_from(0.32927245914677557992e-1_f64) * t49378 + F::cast_from(0.16463622957338778996e-1_f64) * t49382 - F::cast_from(0.29272321618148349057e-1_f64) * t47424 + F::cast_from(0.16463622957338778996e-1_f64) * t47427 - F::cast_from(0.29272321618148349057e-1_f64) * t49386 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t5735 * t9891 + F::cast_from(0.69394917116090352834e-2_f64) * t47432 + F::cast_from(0.9757440539382783019e-2_f64) * t47436 + F::cast_from(0.98781737744032673976e-1_f64) * t49395;
    t49397
}
