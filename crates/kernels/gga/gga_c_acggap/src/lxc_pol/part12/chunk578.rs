//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 578/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk578<F: Float>(t1629: F, t4210: F, t1160: F, t1652: F, t377: F, t1170: F, t151: F, t1530: F, t3057: F, t3059: F, t3063: F, t3067: F, t3071: F, t4182: F, t4185: F, t4188: F, t4191: F, t4192: F, t4194: F, t4198: F, t4200: F, t4203: F, t4206: F) -> F {
    let t4211 = t1629 * t4210;
    let t4213 = F::cast_from(0.13170898365871023197e1_f64) * t1160 * t4211;
    let t4215 = F::cast_from(0.13170898365871023197e1_f64) * t377 * t1652;
    let t4219 = t4182 - F::cast_from(0.13170898365871023197e1_f64) * t4185 - t4188 + t4191 + t3057 + F::cast_from(0.13170898365871023197e1_f64) * t4192 - F::cast_from(0.65854491829355115987e0_f64) * t1170 * t4194 - F::cast_from(0.39512695097613069591e1_f64) * t4198 * t4200 + F::cast_from(0.39512695097613069591e1_f64) * t1530 * t4203 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t4206 + F::cast_from(0.26341796731742046394e1_f64) * t3059 + t4213 - t4215 + F::cast_from(0.13170898365871023197e1_f64) * t3063 + F::cast_from(0.13170898365871023197e1_f64) * t3067 + F::cast_from(0.65854491829355115987e0_f64) * t3071;
    t4219
}
