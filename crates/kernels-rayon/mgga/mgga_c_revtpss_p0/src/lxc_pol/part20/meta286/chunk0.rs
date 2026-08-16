//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1152/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1152(t12051: f64, t357: f64, t12048: f64, t1043: f64, t1089: f64, t3259: f64, t12032: f64, t380: f64, t11620: f64, t378: f64, t359: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12052 = t12051 * t357;
    let t12053 = t12048 * t12052;
    let t12057 = t3259 * t1043 * t1089;
    let t12066 = t380 * t12032;
    let t12070 = t378 * t11620 * t1089;
    let t12073 = t359 * t3259;
    let t12074 = t12073 * t999;
    (t12052, t12053, t12057, t12066, t12070, t12073, t12074)
}
