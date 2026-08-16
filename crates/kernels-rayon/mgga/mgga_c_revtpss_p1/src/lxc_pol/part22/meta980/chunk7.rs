//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3309/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3309(t136: f64, t2457: f64, t2710: f64, t6041: f64, t10535: f64, t5978: f64, t10943: f64, t14663: f64, t18699: f64, t18714: f64, t2754: f64, t39687: f64, t4494: f64, t4504: f64, t4514: f64, t51418: f64, t51422: f64, t51424: f64, t51429: f64, t51434: f64, t51438: f64, t51442: f64, t820: f64) -> f64 {
    let t62716 = t2710 * t6041 * t136 * t2457;
    let t62723 = t10535 * t5978 * t136 * t2457;
    let t62733 = -0.13170898365871023197e1_f64 * t4514 * t4494 * t14663 - 0.65854491829355115987e0_f64 * t820 * t18714 * t2754 + 0.11565819519348392139e-2_f64 * t62716 + 0.13170898365871023197e1_f64 * t4504 * t18699 * t10943 - 0.11565819519348392139e-2_f64 * t62723 + 0.73171657588172351096e-2_f64 * t39687 - 0.39029762157531132076e-1_f64 * t51418 - 0.39029762157531132076e-1_f64 * t51422 - 0.520396828767081761e-2_f64 * t51424 + 0.52039682876708176102e-1_f64 * t51429 + 0.52039682876708176102e-1_f64 * t51434 + 0.21951497276451705328e-1_f64 * t51438 - 0.19514881078765566038e-1_f64 * t51442;
    t62733
}
