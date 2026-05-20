//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3773/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3773<F: Float>(t12717: F, t12723: F, t16696: F, t17289: F, t1770: F, t17818: F, t17879: F, t17883: F, t17884: F, t17958: F, t20856: F, t21164: F, t21257: F, t21442: F, t21443: F, t21480: F, t21551: F, t3601: F, t3666: F, t43350: F, t45666: F, t45707: F, t45738: F, t45852: F, t460: F, t471: F, t489: F, t5216: F, t5458: F, t5477: F, t5481: F, t5487: F, t59737: F, t59749: F, t69655: F, t70235: F, t72098: F, t73: F) -> F {
    let t72140 = -F::cast_from(0.52683593463484092788e1_f64) * t59749 * t17818 - F::cast_from(0.13170898365871023197e1_f64) * t45738 * t69655 * t16696 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t489 * t72098 - F::cast_from(0.13170898365871023197e1_f64) * t3666 * t21551 - F::cast_from(0.13170898365871023197e1_f64) * t12723 * t21480 - F::cast_from(0.26341796731742046394e1_f64) * t5216 * t5477 * t5481 - F::cast_from(0.26341796731742046394e1_f64) * t1770 * t17879 * t5481 - F::cast_from(0.65854491829355115987e0_f64) * t59737 * t70235 * t43350 * t3601 * t471 - F::cast_from(0.13170898365871023197e1_f64) * t17958 * t17884 - F::cast_from(0.26341796731742046394e1_f64) * t17289 * t5487 + F::cast_from(0.52683593463484092788e1_f64) * t45707 * t21443 + F::cast_from(0.52683593463484092788e1_f64) * t45852 * t21443 + F::cast_from(0.52683593463484092788e1_f64) * t12717 * t21164 * t73 * t5458 + F::cast_from(0.52683593463484092788e1_f64) * t12717 * t21257 * t73 * t5458 + F::cast_from(0.26341796731742046394e1_f64) * t12717 * t21442 * t17883 - F::cast_from(0.79025390195226139182e1_f64) * t45666 * t20856 * t73 * t5458;
    t72140
}
