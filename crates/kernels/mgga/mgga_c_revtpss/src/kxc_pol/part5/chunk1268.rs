//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1268/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1268<F: Float>(t1079: F, t20214: F, t1096: F, t6258: F, t1000: F, t1073: F, t1076: F, t11201: F, t16302: F, t16362: F, t1652: F, t1680: F, t1696: F, t20188: F, t20191: F, t20195: F, t20204: F, t20211: F, t3047: F, t3063: F, t4743: F, t4752: F, t4935: F, t4947: F, t6235: F, t6259: F, t995: F) -> F {
    let t20215 = t1079 * t20214;
    let t20218 = t6258 * t1096;
    let t20219 = t1079 * t20218;
    let t20228 = -F::new(0.39512695097613069591e1) * t11201 * t20188 - F::new(0.13170898365871023197e1) * t20191 * t1000 + F::new(0.26341796731742046394e1) * t1076 * t20195 + F::new(0.26341796731742046394e1) * t4935 * t4947 - F::new(0.13170898365871023197e1) * t16362 * t1696 - F::new(0.13170898365871023197e1) * t16302 * t1652 - F::new(0.65854491829355115987e0) * t20204 * t1000 - F::new(0.65854491829355115987e0) * t3047 * t6259 + F::new(0.13170898365871023197e1) * t4743 * t1680 - F::new(0.65854491829355115987e0) * t20211 * t1000 + F::new(0.13170898365871023197e1) * t995 * t20215 + F::new(0.65854491829355115987e0) * t995 * t20219 - F::new(0.65854491829355115987e0) * t3063 * t6259 + F::new(0.65854491829355115987e0) * t6235 * t1073 + F::new(0.26341796731742046394e1) * t4752 * t4947;
    t20228
}
