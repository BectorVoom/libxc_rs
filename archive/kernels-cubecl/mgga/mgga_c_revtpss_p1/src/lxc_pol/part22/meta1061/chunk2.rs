//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3782/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3782<F: Float>(t21164: F, t3153: F, t12723: F, t12744: F, t12751: F, t1280: F, t12975: F, t12987: F, t16772: F, t17178: F, t17183: F, t17192: F, t17307: F, t17811: F, t17840: F, t17869: F, t17884: F, t17945: F, t17949: F, t17955: F, t21416: F, t21436: F, t21452: F, t21491: F, t21495: F, t21579: F, t21596: F, t3746: F, t5465: F, t59650: F, t59817: F, t6720: F, t70741: F, t71606: F) -> (F, F) {
    let t72526 = t21164 * t3153;
    let t72530 = F::cast_from(0.52683593463484092788e1_f64) * t17307 * t16772 + F::cast_from(0.52683593463484092788e1_f64) * t17307 * t17811 + F::cast_from(0.26341796731742046394e1_f64) * t3746 * t21436 - F::cast_from(0.39512695097613069591e1_f64) * t12987 * t1280 * t71606 + F::cast_from(0.52683593463484092788e1_f64) * t17955 * t21596 - F::cast_from(0.26341796731742046394e1_f64) * t12723 * t21491 + F::cast_from(0.26341796731742046394e1_f64) * t17949 * t59650 * t70741 - F::cast_from(0.13170898365871023197e1_f64) * t12975 * t6720 + F::cast_from(0.26341796731742046394e1_f64) * t59817 * t17945 + F::cast_from(0.13170898365871023197e1_f64) * t3746 * t21495 - F::cast_from(0.13170898365871023197e1_f64) * t17192 * t17884 - F::cast_from(0.26341796731742046394e1_f64) * t21579 * t17178 + F::cast_from(0.26341796731742046394e1_f64) * t21452 * t17840 - F::cast_from(0.13170898365871023197e1_f64) * t17183 * t17869 - F::cast_from(0.13170898365871023197e1_f64) * t12744 * t21416 - F::cast_from(0.52683593463484092788e1_f64) * t12751 * t72526 * t5465;
    (t72526, t72530)
}
