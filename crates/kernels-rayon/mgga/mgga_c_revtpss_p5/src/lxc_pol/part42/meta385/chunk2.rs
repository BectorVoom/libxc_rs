//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1272/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1272(t4982: f64, t999: f64, t19501: f64, t1024: f64, t1083: f64, t1087: f64, t11940: f64, t12122: f64, t12149: f64, t16544: f64, t16559: f64, t16566: f64, t19438: f64, t19443: f64, t19447: f64, t19453: f64, t19457: f64, t19463: f64, t19479: f64, t19484: f64, t19488: f64, t19492: f64, t19498: f64, t3223: f64, t3287: f64, t4857: f64, t4954: f64, t4977: f64, t4988: f64, t4992: f64, t4996: f64, t5005: f64, t6368: f64) -> f64 {
    let t19502 = t4982 * t999;
    let t19503 = t19501 * t19502;
    let t19508 = -0.13170898365871023197e1_f64 * t3223 * t6368 - 0.65854491829355115987e0_f64 * t1024 * t19438 - 0.13170898365871023197e1_f64 * t4857 * t5005 - 0.65854491829355115987e0_f64 * t1024 * t19443 + 0.26341796731742046394e1_f64 * t12149 * t19447 + 0.65854491829355115987e0_f64 * t16566 * t19453 - 0.39512695097613069591e1_f64 * t11940 * t19457 + 0.13170898365871023197e1_f64 * t4954 * t4992 - 0.65854491829355115987e0_f64 * t19463 * t1083 + 0.65854491829355115987e0_f64 * t1087 * t19479 - 0.13170898365871023197e1_f64 * t4996 * t19484 + 0.65854491829355115987e0_f64 * t1087 * t19488 - 0.39512695097613069591e1_f64 * t16559 * t19492 - 0.13170898365871023197e1_f64 * t16544 * t4977 - 0.65854491829355115987e0_f64 * t3287 * t19498 - 0.13170898365871023197e1_f64 * t12122 * t19503 + 0.13170898365871023197e1_f64 * t4954 * t4988;
    t19508
}
