//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3783/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3783<F: Float>(t12699: F, t12751: F, t12756: F, t1280: F, t1287: F, t12987: F, t16697: F, t17818: F, t20795: F, t20850: F, t21439: F, t21507: F, t21587: F, t3568: F, t3670: F, t3755: F, t3763: F, t3769: F, t3774: F, t3783: F, t45859: F, t5216: F, t5464: F, t5480: F, t5494: F, t59591: F, t59674: F, t59788: F, t59945: F, t6735: F, t70413: F, t70513: F, t71258: F, t71839: F, t71854: F, t72526: F) -> F {
    let t72572 = F::cast_from(0.26341796731742046394e1_f64) * t12756 * t72526 * t5480 - F::cast_from(0.52683593463484092788e1_f64) * t59788 * t17818 + F::cast_from(0.26341796731742046394e1_f64) * t59674 * t16697 + F::cast_from(0.26341796731742046394e1_f64) * t3670 * t1280 * t70513 + F::cast_from(0.26341796731742046394e1_f64) * t5216 * t5494 - F::cast_from(0.79025390195226139182e1_f64) * t59591 * t21587 + F::cast_from(0.13170898365871023197e1_f64) * t59945 * t21507 + F::cast_from(0.65854491829355115987e0_f64) * t12699 * t6735 + F::cast_from(0.13170898365871023197e1_f64) * t21439 * t3774 + F::cast_from(0.26341796731742046394e1_f64) * t3670 * t1280 * t70413 - F::cast_from(0.39512695097613069591e1_f64) * t12987 * t1280 * t71839 - F::cast_from(0.13170898365871023197e1_f64) * t3755 * t71854 * t1287 - F::cast_from(0.13170898365871023197e1_f64) * t12751 * t71258 * t3769 + F::cast_from(0.65854491829355115987e0_f64) * t12756 * t71258 * t3783 + F::cast_from(0.26341796731742046394e1_f64) * t45859 * t20795 * t5464 * t3568 - F::cast_from(0.65854491829355115987e0_f64) * t20850 * t3763;
    t72572
}
