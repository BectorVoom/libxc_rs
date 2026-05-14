//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1183/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1183<F: Float>(t16423: F, t16475: F, t16526: F, t16589: F, t1079: F, t1071: F, t4746: F, t15669: F, t378: F, t379: F, t994: F, t1695: F, t3268: F, t3066: F, t1000: F, t1076: F, t1097: F, t11128: F, t11210: F, t11214: F, t16362: F, t16371: F, t16374: F, t1652: F, t1696: F, t3047: F, t3060: F, t3067: F, t3076: F, t3264: F, t4747: F, t4773: F, t4778: F, t5016: F) -> (F,) {
    let t16591 = t16423 + t16475 + t16526 + t16589;
    let t16592 = t1079 * t16591;
    let t16597 = t4746 * t1071;
    let t16600 = t15669 * t378;
    let t16603 = t994 * t379;
    let t16604 = t3268 * t1695;
    let t16605 = t16604 * t3066;
    let t16610 = -0.65854491829355115987e0 * t4778 * t3076 - 0.13170898365871023197e1 * t3264 * t5016 - 0.65854491829355115987e0 * t11210 * t1696 - 0.13170898365871023197e1 * t16362 * t1097 - 0.65854491829355115987e0 * t4747 * t3076 - 0.13170898365871023197e1 * t11128 * t1652 - 0.65854491829355115987e0 * t11214 * t1652 - 0.13170898365871023197e1 * t16371 * t1097 - 0.13170898365871023197e1 * t16374 * t1000 - 0.65854491829355115987e0 * t1076 * t16592 - 0.13170898365871023197e1 * t3047 * t4773 - 0.13170898365871023197e1 * t16597 * t1000 + 0.13170898365871023197e1 * t16600 * t3060 - 0.26341796731742046394e1 * t16603 * t16605 + 0.13170898365871023197e1 * t4747 * t3067;
    (t16610,)
}
