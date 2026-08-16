//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3781/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3781<F: Float>(t12709: F, t12751: F, t12756: F, t1280: F, t1285: F, t1287: F, t16695: F, t16757: F, t17178: F, t17188: F, t17192: F, t17880: F, t17905: F, t17949: F, t20795: F, t20956: F, t21448: F, t21456: F, t21468: F, t21471: F, t21500: F, t3584: F, t3588: F, t3670: F, t3755: F, t3767: F, t3769: F, t45744: F, t5446: F, t5458: F, t59657: F, t60008: F, t60019: F, t6695: F, t68669: F, t70824: F, t70944: F, t72050: F, t72165: F) -> F {
    let t72492 = -F::cast_from(0.52683593463484092788e1_f64) * t12751 * t16695 * t72050 + F::cast_from(0.65854491829355115987e0_f64) * t12756 * t20795 * t21471 * t3584 - F::cast_from(0.26341796731742046394e1_f64) * t12709 * t21448 + F::cast_from(0.65854491829355115987e0_f64) * t17949 * t20956 * t45744 - F::cast_from(0.13170898365871023197e1_f64) * t17192 * t17905 + F::cast_from(0.13170898365871023197e1_f64) * t3767 * t72165 * t3769 - F::cast_from(0.26341796731742046394e1_f64) * t21456 * t17178 - F::cast_from(0.26341796731742046394e1_f64) * t60008 * t5446 - F::cast_from(0.26341796731742046394e1_f64) * t59657 * t5446 + F::cast_from(0.26341796731742046394e1_f64) * t3670 * t1280 * t68669 + F::cast_from(0.52683593463484092788e1_f64) * t60019 * t17188 + F::cast_from(0.52683593463484092788e1_f64) * t21500 * t16757 - F::cast_from(0.13170898365871023197e1_f64) * t3755 * t70944 * t5458 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t6695 * t3588 * t1287 - F::cast_from(0.13170898365871023197e1_f64) * t3755 * t70824 * t1287 - F::cast_from(0.13170898365871023197e1_f64) * t17880 * t21468;
    t72492
}
