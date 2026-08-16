//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1098/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1098(t1220: f64, t1914: f64, t316: f64, t879: f64, t1937: f64, t449: f64, t863: f64, t864: f64, t14620: f64, t14621: f64, t14626: f64, t14640: f64, t14642: f64, t14648: f64, t15259: f64, t15262: f64, t15265: f64, t15276: f64, t15278: f64, t1608: f64, t19664: f64, t19668: f64, t19672: f64, t19676: f64, t19678: f64, t4109: f64, t5332: f64) -> f64 {
    let t19688 = t316 * t1220 * t1914 * t879;
    let t19692 = t863 * t449 * t1937 * t864;
    let t19696 = t14620 + 0.52683593463484092788e1_f64 * t15259 - 0.65854491829355115987e0_f64 * t19664 + 0.13170898365871023197e1_f64 * t14621 + t14626 - 0.13170898365871023197e1_f64 * t19668 - 0.26341796731742046394e1_f64 * t19672 - 0.26341796731742046394e1_f64 * t15262 - 0.79025390195226139182e1_f64 * t15265 - 0.79025390195226139182e1_f64 * t19676 - t14640 - 0.13170898365871023197e1_f64 * t19678 - 0.13170898365871023197e1_f64 * t14642 - 0.79025390195226139182e1_f64 * t1608 * t4109 + 0.26341796731742046394e1_f64 * t15276 + 0.39512695097613069591e1_f64 * t15278 + 0.13170898365871023197e1_f64 * t14648 - 0.13170898365871023197e1_f64 * t19688 - 0.13170898365871023197e1_f64 * t19692 - 0.13170898365871023197e1_f64 * t1608 * t5332;
    t19696
}
