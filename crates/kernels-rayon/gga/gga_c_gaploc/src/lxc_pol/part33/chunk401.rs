//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 401/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk401(t656: f64, t90: f64, t256: f64, t64: f64, t1194: f64, t1199: f64, t1201: f64, t1206: f64, t408: f64, t1741: f64, t1762: f64, t1832: f64, t257: f64, t260: f64, t266: f64, t657: f64, t667: f64, t670: f64, t677: f64) -> (f64, f64) {
    let t1913 = t90 * t656;
    let t1916 = t256 * t256;
    let t1917 = 1.0_f64 / t1916;
    let t1918 = t64 * t1917;
    let t1931 = -0.15474205398478635379e-1_f64 * t408 + 0.5833205e-2_f64 * t1194 - 0.16123583333333333333e-2_f64 * t1199 + 0.61251011229312867192e-4_f64 * t1201 - 0.6735290625e-5_f64 * t1206;
    let t1933 = 0.21272952746160294864e-2_f64 * t408 * t257 + 0.42545905492320589728e-2_f64 * t1913 * t667 + 0.63818858238480884592e-2_f64 * t1918 * t1741 - 0.21272952746160294864e-2_f64 * t657 * t1762 - t1832 * t266 - 2.0_f64 * t670 * t677 - t260 * t1931;
    (t1931, t1933)
}
