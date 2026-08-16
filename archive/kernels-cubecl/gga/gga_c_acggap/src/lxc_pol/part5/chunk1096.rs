//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1096/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1096<F: Float>(t1308: F, t5368: F, t1614: F, t4131: F, t1909: F, t848: F, t15151: F, t557: F, t1222: F, t14591: F, t14593: F, t14606: F, t14616: F, t15230: F, t15232: F, t15234: F, t15238: F, t15247: F, t15249: F, t15251: F, t15253: F, t15285: F, t1659: F, t1915: F, t3865: F, t4103: F, t5520: F) -> F {
    let t19637 = t1308 * t5368;
    let t19645 = t4131 * t1614;
    let t19647 = t848 * t1909;
    let t19657 = t15151 * t557;
    let t19662 = F::cast_from(0.13170898365871023197e1_f64) * t3865 * t1915 + F::cast_from(0.13170898365871023197e1_f64) * t19637 + F::cast_from(0.13170898365871023197e1_f64) * t15230 - F::cast_from(0.26341796731742046394e1_f64) * t15232 + F::cast_from(0.26341796731742046394e1_f64) * t15234 + F::cast_from(0.39512695097613069592e1_f64) * t14591 - F::cast_from(0.65854491829355115987e0_f64) * t14593 + F::cast_from(0.13170898365871023197e1_f64) * t15238 + F::cast_from(0.26341796731742046394e1_f64) * t19645 + F::cast_from(0.65854491829355115987e0_f64) * t19647 + F::cast_from(0.52683593463484092788e1_f64) * t15247 - F::cast_from(0.26341796731742046394e1_f64) * t4103 * t1659 + t14606 - F::cast_from(0.13170898365871023197e1_f64) * t15285 * t557 - F::cast_from(0.13170898365871023197e1_f64) * t15249 - F::cast_from(0.26341796731742046394e1_f64) * t15251 + F::cast_from(0.13170898365871023197e1_f64) * t15253 - F::cast_from(0.26341796731742046394e1_f64) * t19657 + F::cast_from(0.13170898365871023197e1_f64) * t5520 * t1222 - F::cast_from(0.39512695097613069592e1_f64) * t14616;
    t19662
}
