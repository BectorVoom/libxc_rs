//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2194/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2194<F: Float>(t10939: F, t10948: F, t10969: F, t10971: F, t14581: F, t14948: F, t14951: F, t14961: F, t1559: F, t18714: F, t18720: F, t18727: F, t18731: F, t18733: F, t18739: F, t18743: F, t18747: F, t18751: F, t18763: F, t6022: F, t820: F) -> F {
    let t23382 = -F::cast_from(0.19756347548806534796e1_f64) * t820 * t18714 * t1559 + F::cast_from(0.58544643236296698113e-1_f64) * t18720 + F::cast_from(0.21951497276451705329e-1_f64) * t14581 - F::cast_from(0.29272321618148349057e-1_f64) * t18727 - F::cast_from(0.29272321618148349057e-1_f64) * t18731 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t14961 * t6022 - F::cast_from(0.58544643236296698113e-1_f64) * t18733 + F::cast_from(0.16463622957338778996e-1_f64) * t18739 + F::cast_from(0.16463622957338778996e-1_f64) * t18743 + F::cast_from(0.32927245914677557992e-1_f64) * t18747 - F::cast_from(0.32927245914677557992e-1_f64) * t18751 + F::cast_from(0.34697458558045176417e-2_f64) * t14948 - F::cast_from(0.39029762157531132076e-1_f64) * t14951 + F::cast_from(0.29272321618148349057e-1_f64) * t18763 + t10939 - t10948 + t10969 - t10971;
    t23382
}
