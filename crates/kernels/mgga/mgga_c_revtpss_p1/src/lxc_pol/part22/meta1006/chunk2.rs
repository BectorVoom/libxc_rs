//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3440/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3440<F: Float>(t11187: F, t11220: F, t11224: F, t15579: F, t16249: F, t16318: F, t16592: F, t19351: F, t19385: F, t19400: F, t19415: F, t19421: F, t20191: F, t20215: F, t3047: F, t3063: F, t3067: F, t3271: F, t4747: F, t4752: F, t4778: F, t4935: F, t6393: F) -> F {
    let t64592 = F::cast_from(0.52683593463484092788e1_f64) * t11187 * t19400 + F::cast_from(0.26341796731742046394e1_f64) * t20191 * t3067 - F::cast_from(0.26341796731742046394e1_f64) * t11224 * t19421 + F::cast_from(0.13170898365871023197e1_f64) * t3047 * t19385 + F::cast_from(0.26341796731742046394e1_f64) * t3063 * t20215 + F::cast_from(0.26341796731742046394e1_f64) * t11187 * t19415 - F::cast_from(0.13170898365871023197e1_f64) * t4752 * t16592 + F::cast_from(0.26341796731742046394e1_f64) * t4935 * t16318 + F::cast_from(0.13170898365871023197e1_f64) * t19351 * t3271 - F::cast_from(0.26341796731742046394e1_f64) * t4747 * t16249 - F::cast_from(0.13170898365871023197e1_f64) * t11220 * t6393 + F::cast_from(0.13170898365871023197e1_f64) * t4778 * t15579;
    t64592
}
