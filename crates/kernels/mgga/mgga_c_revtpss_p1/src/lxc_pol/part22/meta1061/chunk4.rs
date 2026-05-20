//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3784/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3784<F: Float>(t1234: F, t1280: F, t1285: F, t1287: F, t1774: F, t17807: F, t17834: F, t17837: F, t17861: F, t17909: F, t1794: F, t20795: F, t21471: F, t21484: F, t21541: F, t3568: F, t3670: F, t3755: F, t3767: F, t3769: F, t44843: F, t45715: F, t45764: F, t45863: F, t5436: F, t5446: F, t5463: F, t5465: F, t5478: F, t5480: F, t5491: F, t59488: F, t59681: F, t59705: F, t59749: F, t6738: F, t69624: F, t70120: F, t70311: F, t71940: F) -> F {
    let t72618 = -F::cast_from(0.13170898365871023197e1_f64) * t1234 * t59488 * t1774 + F::cast_from(0.26341796731742046394e1_f64) * t3767 * t69624 * t3769 + F::cast_from(0.13170898365871023197e1_f64) * t1285 * t17807 * t1794 * t1287 - F::cast_from(0.13170898365871023197e1_f64) * t5478 * t70311 * t5480 + F::cast_from(0.26341796731742046394e1_f64) * t5436 * t17909 + F::cast_from(0.15805078039045227836e2_f64) * t44843 * t1280 * t70120 - F::cast_from(0.65854491829355115987e0_f64) * t45764 * t6738 - F::cast_from(0.26341796731742046394e1_f64) * t45715 * t21484 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t71940 * t1287 - F::cast_from(0.26341796731742046394e1_f64) * t59749 * t17834 + F::cast_from(0.13170898365871023197e1_f64) * t59681 * t17837 - F::cast_from(0.26341796731742046394e1_f64) * t59705 * t5446 - F::cast_from(0.13170898365871023197e1_f64) * t45863 * t20795 * t21471 * t3568 + F::cast_from(0.26341796731742046394e1_f64) * t5463 * t70311 * t5465 + F::cast_from(0.13170898365871023197e1_f64) * t3670 * t21541 * t3568 + F::cast_from(0.26341796731742046394e1_f64) * t17861 * t5491;
    t72618
}
