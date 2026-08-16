//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1043/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1043<F: Float>(t136814: F, t136847: F, t137037: F, t145188: F, t145192: F, t145200: F, t145205: F, t145209: F, t145223: F, t22597: F, t22603: F, t22736: F, t25658: F, t25685: F, t3025: F, t3099: F, t32133: F, t32140: F, t32169: F, t32259: F, t34472: F, t378: F, t5538: F, t92314: F) -> F {
    let t145234 = -F::cast_from(0.61277550024922479209e-6_f64) * t92314 * t145200 + F::cast_from(0.1721820212247325051e-5_f64) * t22597 * t145205 + F::cast_from(0.25845121844514357744e-4_f64) * t136814 * t145209 - F::cast_from(0.22979081259345929704e-6_f64) * t22736 * t32133 * t25658 - F::cast_from(0.17608347349624143343e-1_f64) * t32169 * t32140 * t378 * t3099 + F::cast_from(0.93911185864662097829e-1_f64) * t32169 * t136847 * t34472 - F::cast_from(0.11738898233082762229e-1_f64) * t145223 - F::cast_from(0.25845121844514357744e-4_f64) * t22603 * t145188 - F::cast_from(0.51690243689028715488e-5_f64) * t5538 * t145192 - F::cast_from(0.17782141943527538963e-1_f64) * t32259 * t25685 + F::cast_from(0.51690243689028715488e-4_f64) * t22597 * t137037 * t3025;
    t145234
}
