//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1314/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1314(t119: f64, t6413: f64, t5511: f64, t868: f64, t852: f64, t1914: f64, t3874: f64, t5384: f64, t871: f64, t5378: f64, t5385: f64, t1215: f64, t1221: f64, t14695: f64, t150: f64, t187: f64, t18872: f64, t18875: f64, t1937: f64, t24340: f64, t3875: f64, t446: f64, t464: f64, t6558: f64) -> f64 {
    let t24516 = t119 * t6413;
    let t24519 = t868 * t5511;
    let t24521 = t852 * t5511;
    let t24531 = t5384 * t3874 * t1914 * t871;
    let t24534 = t5384 * t5385 * t5378;
    let t24540 = 0.13170898365871023197e1_f64 * t18872 + 0.26341796731742046394e1_f64 * t18875 - 0.13170898365871023197e1_f64 * t24516 * t464 - t14695 + 0.13170898365871023197e1_f64 * t24519 + 0.13170898365871023197e1_f64 * t24521 - 0.13170898365871023197e1_f64 * t1215 * t6558 + 0.65854491829355115987e0_f64 * t119 * t24340 * t150 * t187 + 0.79025390195226139182e1_f64 * t24531 - 0.52683593463484092788e1_f64 * t24534 - 0.39512695097613069591e1_f64 * t446 * t3875 * t1937 * t1221;
    t24540
}
