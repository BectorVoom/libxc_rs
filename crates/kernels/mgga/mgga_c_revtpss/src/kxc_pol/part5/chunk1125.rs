//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1125/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1125<F: Float>(t4982: F, t999: F, t19501: F, t1024: F, t1083: F, t1087: F, t11940: F, t12122: F, t12149: F, t16544: F, t16559: F, t16566: F, t19438: F, t19443: F, t19447: F, t19453: F, t19457: F, t19463: F, t19479: F, t19484: F, t19488: F, t19492: F, t19498: F, t3223: F, t3287: F, t4857: F, t4954: F, t4977: F, t4988: F, t4992: F, t4996: F, t5005: F, t6368: F) -> (F,) {
    let t19502 = t4982 * t999;
    let t19503 = t19501 * t19502;
    let t19508 = -0.13170898365871023197e1 * t3223 * t6368 - 0.65854491829355115987e0 * t1024 * t19438 - 0.13170898365871023197e1 * t4857 * t5005 - 0.65854491829355115987e0 * t1024 * t19443 + 0.26341796731742046394e1 * t12149 * t19447 + 0.65854491829355115987e0 * t16566 * t19453 - 0.39512695097613069591e1 * t11940 * t19457 + 0.13170898365871023197e1 * t4954 * t4992 - 0.65854491829355115987e0 * t19463 * t1083 + 0.65854491829355115987e0 * t1087 * t19479 - 0.13170898365871023197e1 * t4996 * t19484 + 0.65854491829355115987e0 * t1087 * t19488 - 0.39512695097613069591e1 * t16559 * t19492 - 0.13170898365871023197e1 * t16544 * t4977 - 0.65854491829355115987e0 * t3287 * t19498 - 0.13170898365871023197e1 * t12122 * t19503 + 0.13170898365871023197e1 * t4954 * t4988;
    (t19508,)
}
