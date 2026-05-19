//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1313/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1313<F: Float>(t1915: F, t3892: F, t6434: F, t880: F, t315: F, t5386: F, t546: F, t1220: F, t1221: F, t1264: F, t14652: F, t14671: F, t15290: F, t15293: F, t15295: F, t15299: F, t15303: F, t15314: F, t1608: F, t1914: F, t1937: F, t19704: F, t19706: F, t19708: F, t19711: F, t19738: F, t19773: F, t19797: F, t19824: F, t19857: F, t19887: F, t20134: F, t20161: F, t20188: F, t20216: F, t20237: F, t24368: F, t24395: F, t24415: F, t24441: F, t24469: F, t4119: F, t446: F, t449: F, t5331: F, t5340: F, t556: F) -> F {
    let t24482 = t3892 * t1915;
    let t24485 = t6434 * t880;
    let t24491 = t315 * t546 * t5386;
    let t24499 = F::cast_from(0.15805078039045227836e2_f64) * t446 * t14652 * t1914 * t1221 + F::cast_from(0.26341796731742046394e1_f64) * t1608 * t5340 + F::cast_from(0.13170898365871023197e1_f64) * t19704 + F::cast_from(0.26341796731742046394e1_f64) * t19706 - F::cast_from(0.13170898365871023197e1_f64) * t19708 - F::cast_from(0.39512695097613069591e1_f64) * t15290 + F::cast_from(0.26341796731742046394e1_f64) * t446 * t1220 * t19711 + F::cast_from(0.79025390195226139182e1_f64) * t15293 - F::cast_from(0.65854491829355115987e0_f64) * t446 * t449 * (t19738 + t19773 + t19797 + t19824 + t19857 + t19887 + t20134 + t20161 + t20188 + t20216 + t20237 + t24368 + t24395 + t24415 + t24441 + t24469) + F::cast_from(0.13170898365871023197e1_f64) * t446 * t1220 * t1937 * t1264 - F::cast_from(0.13170898365871023197e1_f64) * t15295 + F::cast_from(0.13170898365871023197e1_f64) * t24482 + F::cast_from(0.79025390195226139182e1_f64) * t15299 - F::cast_from(0.65854491829355115987e0_f64) * t24485 + F::cast_from(0.13170898365871023197e1_f64) * t15303 - F::cast_from(0.39512695097613069591e1_f64) * t15314 + F::cast_from(0.13170898365871023197e1_f64) * t14671 - F::cast_from(0.52683593463484092788e1_f64) * t24491 + F::cast_from(0.52683593463484092788e1_f64) * t1608 * t4119 + F::cast_from(0.26341796731742046394e1_f64) * t446 * t1220 * t556 * t5331;
    t24499
}
