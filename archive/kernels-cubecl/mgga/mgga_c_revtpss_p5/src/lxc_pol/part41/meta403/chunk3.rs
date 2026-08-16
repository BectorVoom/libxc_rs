//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1397/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1397<F: Float>(t1770: F, t5462: F, t12050: F, t1248: F, t471: F, t20956: F, t3153: F, t6688: F, t5465: F, t12709: F, t12723: F, t12751: F, t12756: F, t1285: F, t17192: F, t17861: F, t17949: F, t17958: F, t1822: F, t21465: F, t21468: F, t21473: F, t21480: F, t21484: F, t21491: F, t21495: F, t3746: F, t3755: F, t5436: F, t5446: F, t5459: F, t5463: F, t5466: F, t5478: F, t5491: F, t6717: F, t6731: F) -> (F, F) {
    let t21500 = t1770 * t5462;
    let t21506 = t12050 * t1248 * t471;
    let t21507 = t20956 * t21506;
    let t21512 = t6688 * t3153;
    let t21513 = t21512 * t5465;
    let t21516 = F::cast_from(0.13170898365871023197e1_f64) * t5463 * t21465 - F::cast_from(0.65854491829355115987e0_f64) * t5478 * t21468 + F::cast_from(0.65854491829355115987e0_f64) * t12756 * t21473 + F::cast_from(0.13170898365871023197e1_f64) * t5436 * t5491 + F::cast_from(0.13170898365871023197e1_f64) * t17861 * t1822 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t21480 - F::cast_from(0.13170898365871023197e1_f64) * t12751 * t21484 - F::cast_from(0.13170898365871023197e1_f64) * t12709 * t6717 - F::cast_from(0.13170898365871023197e1_f64) * t12723 * t6717 - F::cast_from(0.13170898365871023197e1_f64) * t3755 * t21491 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t21495 - F::cast_from(0.13170898365871023197e1_f64) * t17192 * t5446 + F::cast_from(0.26341796731742046394e1_f64) * t21500 * t5466 + F::cast_from(0.13170898365871023197e1_f64) * t3746 * t6731 + F::cast_from(0.65854491829355115987e0_f64) * t17949 * t21507 - F::cast_from(0.13170898365871023197e1_f64) * t17958 * t5459 - F::cast_from(0.26341796731742046394e1_f64) * t12751 * t21513;
    (t21512, t21516)
}
