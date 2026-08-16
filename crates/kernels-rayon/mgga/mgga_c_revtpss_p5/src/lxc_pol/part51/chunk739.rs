//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 739/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk739(t1940: f64, t30: f64, t8490: f64, t8494: f64, t1032: f64, t1982: f64, t359: f64, t365: f64, t369: f64) -> (f64, f64, f64, f64, f64) {
    let t8498 = t1940 * t8490 * t30 / 2.0_f64 - t1940 * t8494 * t30 / 2.0_f64;
    let t8499 = t1982 * t1032;
    let t8500 = t359 * t365;
    let t8501 = t8500 * t369;
    let t8502 = t8499 * t8501;
    (t8498, t8499, t8500, t8501, t8502)
}
