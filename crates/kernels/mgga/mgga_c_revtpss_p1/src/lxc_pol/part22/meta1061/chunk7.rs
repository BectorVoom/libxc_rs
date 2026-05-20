//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3787/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3787<F: Float>(t3603: F, t43350: F, t13126: F, t1811: F, t460: F, t1248: F, t12709: F, t12713: F, t12751: F, t12756: F, t1285: F, t1287: F, t12987: F, t17331: F, t17834: F, t17837: F, t17848: F, t17855: F, t17875: F, t17952: F, t1825: F, t20747: F, t21342: F, t21439: F, t21459: F, t21512: F, t3601: F, t3759: F, t3769: F, t3778: F, t3783: F, t45654: F, t45659: F, t45697: F, t45859: F, t45863: F, t5459: F, t59657: F, t59674: F, t59730: F, t59788: F, t6717: F, t70235: F, t71480: F, t72303: F) -> F {
    let t72724 = t43350 * t3603;
    let t72732 = t460 * t13126 * t1811;
    let t72757 = -F::cast_from(0.79025390195226139182e1_f64) * t12987 * t3759 * t20747 + F::cast_from(0.26341796731742046394e1_f64) * t45859 * t71480 * t3769 - F::cast_from(0.13170898365871023197e1_f64) * t45863 * t71480 * t3783 - F::cast_from(0.13170898365871023197e1_f64) * t45697 * t6717 - F::cast_from(0.26341796731742046394e1_f64) * t59788 * t17834 + F::cast_from(0.13170898365871023197e1_f64) * t59674 * t17837 + F::cast_from(0.92196288561097162379e1_f64) * t59730 * t70235 * t72724 * t3601 - F::cast_from(0.26341796731742046394e1_f64) * t59657 * t5459 + F::cast_from(0.13170898365871023197e1_f64) * t72732 * t17952 + F::cast_from(0.13170898365871023197e1_f64) * t17331 * t1825 - F::cast_from(0.13170898365871023197e1_f64) * t12709 * t21459 + F::cast_from(0.13170898365871023197e1_f64) * t1285 * t21342 * t1248 * t1287 + F::cast_from(0.65854491829355115987e0_f64) * t21439 * t3778 - F::cast_from(0.26341796731742046394e1_f64) * t12751 * t21512 * t12713 - F::cast_from(0.79025390195226139182e1_f64) * t45654 * t72303 * t17848 + F::cast_from(0.79025390195226139182e1_f64) * t45659 * t72303 * t17855 + F::cast_from(0.13170898365871023197e1_f64) * t12756 * t21512 * t17875;
    t72757
}
