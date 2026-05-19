//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1287/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1287<F: Float>(t11227: F, t11302: F, t11305: F, t1208: F, t1209: F, t2296: F, t2318: F, t27795: F, t31327: F, t31329: F, t31331: F, t31333: F, t31335: F, t31337: F, t31339: F, t3135: F, t3136: F, t3807: F, t3819: F, t6266: F, t6300: F, t6323: F, t8071: F, t889: F, t9878: F, t9929: F) -> F {
    let t31345 = -F::cast_from(0.70178683471615754484e1_f64) * t8071 * t9878 - F::cast_from(0.14035736694323150897e2_f64) * t6323 * t11227 * t889 + F::cast_from(0.10526802520742363173e2_f64) * t2318 * t3807 * t3135 - F::cast_from(0.35089341735807877242e1_f64) * t6266 * t11302 - F::cast_from(0.35089341735807877242e1_f64) * t2296 * t3136 * t3819 - F::cast_from(0.35089341735807877242e1_f64) * t2296 * t1209 * t9929 + t31327 - t31329 + t31331 - t31333 + t31335 - t31337 - t31339 + F::cast_from(0.51947577317044391277e2_f64) * t6300 * t11305 + F::cast_from(0.51947577317044391277e2_f64) * t2318 * t27795 * t1208;
    t31345
}
