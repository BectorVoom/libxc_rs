//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1937/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1937<F: Float>(t19462: F, t378: F, t1695: F, t4772: F, t1079: F, t1096: F, t6258: F, t1000: F, t1073: F, t1076: F, t11201: F, t16302: F, t16362: F, t1652: F, t1680: F, t1696: F, t20188: F, t20191: F, t20195: F, t20204: F, t3047: F, t3063: F, t4743: F, t4752: F, t4935: F, t4947: F, t6235: F, t6259: F, t995: F) -> (F, F, F, F, F, F) {
    let t20211 = t19462 * t378;
    let t20214 = t4772 * t1695;
    let t20215 = t1079 * t20214;
    let t20218 = t6258 * t1096;
    let t20219 = t1079 * t20218;
    let t20228 = -F::cast_from(0.39512695097613069591e1_f64) * t11201 * t20188 - F::cast_from(0.13170898365871023197e1_f64) * t20191 * t1000 + F::cast_from(0.26341796731742046394e1_f64) * t1076 * t20195 + F::cast_from(0.26341796731742046394e1_f64) * t4935 * t4947 - F::cast_from(0.13170898365871023197e1_f64) * t16362 * t1696 - F::cast_from(0.13170898365871023197e1_f64) * t16302 * t1652 - F::cast_from(0.65854491829355115987e0_f64) * t20204 * t1000 - F::cast_from(0.65854491829355115987e0_f64) * t3047 * t6259 + F::cast_from(0.13170898365871023197e1_f64) * t4743 * t1680 - F::cast_from(0.65854491829355115987e0_f64) * t20211 * t1000 + F::cast_from(0.13170898365871023197e1_f64) * t995 * t20215 + F::cast_from(0.65854491829355115987e0_f64) * t995 * t20219 - F::cast_from(0.65854491829355115987e0_f64) * t3063 * t6259 + F::cast_from(0.65854491829355115987e0_f64) * t6235 * t1073 + F::cast_from(0.26341796731742046394e1_f64) * t4752 * t4947;
    (t20211, t20214, t20215, t20218, t20219, t20228)
}
