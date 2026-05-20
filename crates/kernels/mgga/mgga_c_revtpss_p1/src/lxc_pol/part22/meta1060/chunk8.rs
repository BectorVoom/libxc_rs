//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3779/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3779<F: Float>(t3727: F, t6628: F, t3766: F, t6564: F, t17191: F, t5219: F, t21342: F, t473: F, t1214: F, t1234: F, t12756: F, t12975: F, t16695: F, t16750: F, t16757: F, t17821: F, t17840: F, t17876: F, t17880: F, t17945: F, t21452: F, t21456: F, t21500: F, t21542: F, t21558: F, t21562: F, t3666: F, t3746: F, t3756: F, t3767: F, t3769: F, t3770: F, t460: F, t5245: F, t5412: F, t5459: F, t5466: F, t5486: F, t59705: F, t60019: F, t6723: F, t70712: F) -> (F, F) {
    let t72359 = t3727 * t6628;
    let t72370 = t6564 * t3766;
    let t72386 = t5219 * t17191;
    let t72397 = t473 * t21342;
    let t72404 = F::cast_from(0.13170898365871023197e1_f64) * t3767 * t72359 * t3769 + F::cast_from(0.52683593463484092788e1_f64) * t21452 * t16757 - F::cast_from(0.13170898365871023197e1_f64) * t1234 * t5486 * t16750 - F::cast_from(0.65854491829355115987e0_f64) * t12975 * t6723 + F::cast_from(0.13170898365871023197e1_f64) * t72370 * t3770 - F::cast_from(0.26341796731742046394e1_f64) * t17880 * t21558 + F::cast_from(0.52683593463484092788e1_f64) * t460 * t3766 * t5412 * t5466 - F::cast_from(0.13170898365871023197e1_f64) * t21456 * t17876 - F::cast_from(0.26341796731742046394e1_f64) * t1234 * t17821 * t5245 + F::cast_from(0.26341796731742046394e1_f64) * t60019 * t17945 - F::cast_from(0.26341796731742046394e1_f64) * t72386 * t3756 - F::cast_from(0.26341796731742046394e1_f64) * t59705 * t5459 + F::cast_from(0.26341796731742046394e1_f64) * t21500 * t17840 - F::cast_from(0.13170898365871023197e1_f64) * t3666 * t21542 + F::cast_from(0.13170898365871023197e1_f64) * t3746 * t21562 - F::cast_from(0.13170898365871023197e1_f64) * t1234 * t72397 * t1214 + F::cast_from(0.26341796731742046394e1_f64) * t12756 * t16695 * t70712;
    (t72359, t72404)
}
