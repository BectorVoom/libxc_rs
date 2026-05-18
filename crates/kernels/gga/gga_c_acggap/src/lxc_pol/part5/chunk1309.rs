//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1309/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1309<F: Float>(t1160: F, t1539: F, t19807: F, t4210: F, t6482: F, t14518: F, t14525: F, t14528: F, t14534: F, t151: F, t1530: F, t1533: F, t1629: F, t19161: F, t19172: F, t19176: F, t19179: F, t19181: F, t20228: F, t24113: F, t4198: F, t6551: F, t955: F) -> F {
    let t24388 = t1160 * t19807 * t1539;
    let t24392 = t1160 * t6482 * t4210;
    let t24395 = -F::new(0.65854491829355115987e0) * t151 * t6551 * t955 - F::new(0.15805078039045227836e2) * t4198 * t1629 * t24113 + F::new(0.26341796731742046394e1) * t1530 * t20228 * t1533 + F::new(0.52683593463484092788e1) * t19161 - F::new(0.26341796731742046394e1) * t14518 + F::new(0.26341796731742046394e1) * t19172 + F::new(0.79025390195226139182e1) * t14525 + F::new(0.65854491829355115987e0) * t14528 + F::new(0.79025390195226139182e1) * t19176 + F::new(0.13170898365871023197e1) * t19179 + F::new(0.26341796731742046394e1) * t24388 - F::new(0.13170898365871023197e1) * t19181 + F::new(0.13170898365871023197e1) * t24392 - F::new(0.13170898365871023197e1) * t14534;
    t24395
}
