//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1127/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1127<F: Float>(t406: F, t6263: F, t394: F, t6413: F, t1160: F, t1539: F, t19748: F, t377: F, t6523: F, t1934: F, t848: F, t1170: F, t12385: F, t1411: F, t151: F, t1530: F, t1629: F, t19038: F, t19040: F, t19042: F, t19045: F, t19048: F, t19053: F, t19060: F, t407: F, t4166: F) -> (F, F) {
    let t20138 = t6263 * t406;
    let t20142 = t394 * t6413;
    let t20149 = t1160 * t19748 * t1539;
    let t20157 = t377 * t6523;
    let t20159 = t848 * t1934;
    let t20161 = F::new(0.52683593463484092788e1) * t19038 - F::new(0.13170898365871023197e1) * t19040 - F::new(0.26341796731742046394e1) * t19042 + F::new(0.15805078039045227836e2) * t1530 * t1629 * t20138 - F::new(0.13170898365871023197e1) * t151 * t20142 * t407 - F::new(0.13170898365871023197e1) * t19045 - F::new(0.52683593463484092788e1) * t19048 + F::new(0.13170898365871023197e1) * t20149 - F::new(0.26341796731742046394e1) * t19053 - F::new(0.26341796731742046394e1) * t1170 * t4166 * t1411 - F::new(0.65854491829355115987e0) * t12385 + F::new(0.39512695097613069591e1) * t19060 - F::new(0.26341796731742046394e1) * t20157 + F::new(0.65854491829355115987e0) * t20159;
    (t20138, t20161)
}
