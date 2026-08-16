//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2843/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2843(t10697: f64, t23114: f64, t236: f64, t807: f64, t23267: f64, t2703: f64, t40850: f64, t51059: f64, t51061: f64, t51074: f64, t51079: f64, t51081: f64, t51083: f64, t51086: f64, t51089: f64, t51093: f64, t51096: f64, t62162: f64, t62168: f64, t62176: f64, t62178: f64, t62188: f64) -> f64 {
    let t76856 = t807 * t236 * t10697 * t23114;
    let t76858 = t2703 * t23267;
    let t76860 = t51059 + t51061 - 0.24009450146119052704e-1_f64 * t62162 - 7.0_f64 / 16.0_f64 * t62168 + 0.15246000842785598467e-3_f64 * t62176 + 0.30011812682648815881e-2_f64 * t62178 - 0.18007087609589289528e-1_f64 * t62188 - 0.91464571985215438873e-3_f64 * t51074 - t51079 + 0.27107389498472794074e-4_f64 * t51081 + 0.13553694749236397038e-5_f64 * t51083 + 0.5421477899694558815e-4_f64 * t51086 + 0.54214778996945588148e-4_f64 * t51089 - t51093 + 0.85748036236139473942e-3_f64 * t76856 - t40850 + 7.0_f64 / 144.0_f64 * t76858 - t51096;
    t76860
}
