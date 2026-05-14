//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1157/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1157<F: Float>(t19456: F, t996: F, t1678: F, t4746: F, t1695: F, t5015: F, t3269: F, t6343: F, t994: F, t19462: F, t378: F, t4772: F, t1079: F, t1096: F, t6258: F, t1000: F, t1073: F, t1076: F, t11201: F, t16302: F, t16362: F, t1652: F, t1680: F, t1696: F, t3047: F, t3063: F, t4743: F, t4752: F, t4935: F, t4947: F, t6235: F, t6259: F, t995: F) -> (F,) {
    let t20188 = t996 * t19456;
    let t20191 = t4746 * t1678;
    let t20194 = t1695 * t5015;
    let t20195 = t3269 * t20194;
    let t20204 = t994 * t6343;
    let t20211 = t19462 * t378;
    let t20214 = t4772 * t1695;
    let t20215 = t1079 * t20214;
    let t20218 = t6258 * t1096;
    let t20219 = t1079 * t20218;
    let t20228 = -0.39512695097613069591e1 * t11201 * t20188 - 0.13170898365871023197e1 * t20191 * t1000 + 0.26341796731742046394e1 * t1076 * t20195 + 0.26341796731742046394e1 * t4935 * t4947 - 0.13170898365871023197e1 * t16362 * t1696 - 0.13170898365871023197e1 * t16302 * t1652 - 0.65854491829355115987e0 * t20204 * t1000 - 0.65854491829355115987e0 * t3047 * t6259 + 0.13170898365871023197e1 * t4743 * t1680 - 0.65854491829355115987e0 * t20211 * t1000 + 0.13170898365871023197e1 * t995 * t20215 + 0.65854491829355115987e0 * t995 * t20219 - 0.65854491829355115987e0 * t3063 * t6259 + 0.65854491829355115987e0 * t6235 * t1073 + 0.26341796731742046394e1 * t4752 * t4947;
    (t20228,)
}
