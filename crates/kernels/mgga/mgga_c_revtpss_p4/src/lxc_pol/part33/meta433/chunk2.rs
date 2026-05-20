//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1551/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1551<F: Float>(t4982: F, t999: F, t19501: F, t1024: F, t1083: F, t1087: F, t11940: F, t12122: F, t12149: F, t16544: F, t16559: F, t16566: F, t19438: F, t19443: F, t19447: F, t19453: F, t19457: F, t19463: F, t19479: F, t19484: F, t19488: F, t19492: F, t19498: F, t3223: F, t3287: F, t4857: F, t4954: F, t4977: F, t4988: F, t4992: F, t4996: F, t5005: F, t6368: F) -> F {
    let t19502 = t4982 * t999;
    let t19503 = t19501 * t19502;
    let t19508 = -F::cast_from(0.13170898365871023197e1_f64) * t3223 * t6368 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t19438 - F::cast_from(0.13170898365871023197e1_f64) * t4857 * t5005 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t19443 + F::cast_from(0.26341796731742046394e1_f64) * t12149 * t19447 + F::cast_from(0.65854491829355115987e0_f64) * t16566 * t19453 - F::cast_from(0.39512695097613069591e1_f64) * t11940 * t19457 + F::cast_from(0.13170898365871023197e1_f64) * t4954 * t4992 - F::cast_from(0.65854491829355115987e0_f64) * t19463 * t1083 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t19479 - F::cast_from(0.13170898365871023197e1_f64) * t4996 * t19484 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t19488 - F::cast_from(0.39512695097613069591e1_f64) * t16559 * t19492 - F::cast_from(0.13170898365871023197e1_f64) * t16544 * t4977 - F::cast_from(0.65854491829355115987e0_f64) * t3287 * t19498 - F::cast_from(0.13170898365871023197e1_f64) * t12122 * t19503 + F::cast_from(0.13170898365871023197e1_f64) * t4954 * t4988;
    t19508
}
