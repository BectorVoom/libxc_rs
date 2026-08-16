//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 868/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk868(t1437: f64, t1883: f64, t213: f64, t4082: f64, t4085: f64, t4099: f64, t4113: f64, t4114: f64, t546: f64, t5738: f64, t5742: f64, t5761: f64, t5765: f64, t5767: f64, t6844: f64, t6862: f64, t6874: f64, t6888: f64, t820: f64) -> f64 {
    let t6918 = t4082 - t4085 + 0.10975748638225852664e-1_f64 * t5738 - 0.10975748638225852664e-1_f64 * t5761 + t4099 - 0.19514881078765566038e-1_f64 * t5742 + 0.19514881078765566038e-1_f64 * t5765 - t4113 + 0.13170898365871023197e1_f64 * t820 * t4114 * t6862 - 0.13170898365871023197e1_f64 * t820 * t5767 * t1883 - 0.65854491829355115987e0_f64 * t820 * t1437 * t6844 - 0.65854491829355115987e0_f64 * t820 * t1437 * t6874 + 0.65854491829355115987e0_f64 * t213 * t546 * t6888;
    t6918
}
