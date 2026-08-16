//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2843/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2843<F: Float>(t10697: F, t23114: F, t236: F, t807: F, t23267: F, t2703: F, t40850: F, t51059: F, t51061: F, t51074: F, t51079: F, t51081: F, t51083: F, t51086: F, t51089: F, t51093: F, t51096: F, t62162: F, t62168: F, t62176: F, t62178: F, t62188: F) -> F {
    let t76856 = t807 * t236 * t10697 * t23114;
    let t76858 = t2703 * t23267;
    let t76860 = t51059 + t51061 - F::cast_from(0.24009450146119052704e-1_f64) * t62162 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t62168 + F::cast_from(0.15246000842785598467e-3_f64) * t62176 + F::cast_from(0.30011812682648815881e-2_f64) * t62178 - F::cast_from(0.18007087609589289528e-1_f64) * t62188 - F::cast_from(0.91464571985215438873e-3_f64) * t51074 - t51079 + F::cast_from(0.27107389498472794074e-4_f64) * t51081 + F::cast_from(0.13553694749236397038e-5_f64) * t51083 + F::cast_from(0.5421477899694558815e-4_f64) * t51086 + F::cast_from(0.54214778996945588148e-4_f64) * t51089 - t51093 + F::cast_from(0.85748036236139473942e-3_f64) * t76856 - t40850 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t76858 - t51096;
    t76860
}
