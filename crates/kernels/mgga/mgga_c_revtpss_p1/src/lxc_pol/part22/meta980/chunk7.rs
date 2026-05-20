//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3309/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3309<F: Float>(t136: F, t2457: F, t2710: F, t6041: F, t10535: F, t5978: F, t10943: F, t14663: F, t18699: F, t18714: F, t2754: F, t39687: F, t4494: F, t4504: F, t4514: F, t51418: F, t51422: F, t51424: F, t51429: F, t51434: F, t51438: F, t51442: F, t820: F) -> F {
    let t62716 = t2710 * t6041 * t136 * t2457;
    let t62723 = t10535 * t5978 * t136 * t2457;
    let t62733 = -F::cast_from(0.13170898365871023197e1_f64) * t4514 * t4494 * t14663 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t18714 * t2754 + F::cast_from(0.11565819519348392139e-2_f64) * t62716 + F::cast_from(0.13170898365871023197e1_f64) * t4504 * t18699 * t10943 - F::cast_from(0.11565819519348392139e-2_f64) * t62723 + F::cast_from(0.73171657588172351096e-2_f64) * t39687 - F::cast_from(0.39029762157531132076e-1_f64) * t51418 - F::cast_from(0.39029762157531132076e-1_f64) * t51422 - F::cast_from(0.520396828767081761e-2_f64) * t51424 + F::cast_from(0.52039682876708176102e-1_f64) * t51429 + F::cast_from(0.52039682876708176102e-1_f64) * t51434 + F::cast_from(0.21951497276451705328e-1_f64) * t51438 - F::cast_from(0.19514881078765566038e-1_f64) * t51442;
    t62733
}
