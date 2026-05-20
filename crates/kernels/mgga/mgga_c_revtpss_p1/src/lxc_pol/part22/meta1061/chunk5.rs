//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3785/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3785<F: Float>(t21257: F, t3153: F, t1204: F, t12702: F, t12751: F, t12756: F, t16768: F, t17172: F, t17175: F, t17822: F, t17846: F, t17848: F, t17853: F, t17855: F, t17861: F, t17941: F, t20956: F, t21427: F, t21455: F, t21535: F, t21554: F, t3666: F, t3746: F, t45675: F, t45679: F, t5326: F, t5436: F, t5465: F, t5470: F, t5480: F, t5481: F, t70890: F) -> F {
    let t72627 = t21257 * t3153;
    let t72659 = -F::cast_from(0.13170898365871023197e1_f64) * t5326 * t16768 - F::cast_from(0.13170898365871023197e1_f64) * t5326 * t17175 - F::cast_from(0.13170898365871023197e1_f64) * t3666 * t21554 + F::cast_from(0.13170898365871023197e1_f64) * t3746 * t21535 - F::cast_from(0.52683593463484092788e1_f64) * t12751 * t72627 * t5465 + F::cast_from(0.26341796731742046394e1_f64) * t12756 * t72627 * t5480 + F::cast_from(0.39512695097613069591e1_f64) * t17846 * t20956 * t45675 + F::cast_from(0.26341796731742046394e1_f64) * t12702 * t21427 + F::cast_from(0.39512695097613069591e1_f64) * t17846 * t70890 * t17848 - F::cast_from(0.39512695097613069591e1_f64) * t17853 * t70890 * t17855 - F::cast_from(0.26341796731742046394e1_f64) * t1204 * t21455 * t5481 + F::cast_from(0.13170898365871023197e1_f64) * t5436 * t17172 - F::cast_from(0.26341796731742046394e1_f64) * t5326 * t17941 - F::cast_from(0.39512695097613069591e1_f64) * t17853 * t20956 * t45679 + F::cast_from(0.26341796731742046394e1_f64) * t17861 * t5470 - F::cast_from(0.26341796731742046394e1_f64) * t5326 * t17822;
    t72659
}
