//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3786/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3786<F: Float>(t3140: F, t3566: F, t13147: F, t1811: F, t460: F, t1243: F, t12699: F, t12751: F, t1287: F, t12966: F, t16697: F, t16751: F, t16763: F, t17289: F, t1774: F, t17818: F, t17826: F, t17849: F, t17861: F, t17921: F, t20795: F, t21430: F, t3584: F, t3596: F, t3755: F, t3781: F, t3782: F, t3783: F, t45710: F, t487: F, t5326: F, t5412: F, t5436: F, t5449: F, t5464: F, t5474: F, t5481: F, t6727: F, t6731: F, t70693: F, t72359: F) -> F {
    let t72661 = t3566 * t3140;
    let t72686 = t460 * t13147 * t1811;
    let t72708 = F::cast_from(0.10536718692696818558e2_f64) * t72661 * t3596 * t487 * t1774 * t17818 - F::cast_from(0.52683593463484092788e1_f64) * t72661 * t1243 * t487 * t1774 * t16697 - F::cast_from(0.65854491829355115987e0_f64) * t3782 * t72359 * t3783 - F::cast_from(0.13170898365871023197e1_f64) * t12751 * t20795 * t5464 * t3584 - F::cast_from(0.13170898365871023197e1_f64) * t5326 * t16751 - F::cast_from(0.26341796731742046394e1_f64) * t460 * t3781 * t5412 * t5481 + F::cast_from(0.79025390195226139182e1_f64) * t72686 * t17849 + F::cast_from(0.13170898365871023197e1_f64) * t5436 * t17921 - F::cast_from(0.26341796731742046394e1_f64) * t3755 * t70693 * t1287 + F::cast_from(0.13170898365871023197e1_f64) * t45710 * t6727 + F::cast_from(0.26341796731742046394e1_f64) * t5436 * t17826 + F::cast_from(0.13170898365871023197e1_f64) * t12699 * t6731 + F::cast_from(0.13170898365871023197e1_f64) * t5436 * t16763 + F::cast_from(0.26341796731742046394e1_f64) * t17861 * t5474 - F::cast_from(0.26341796731742046394e1_f64) * t17289 * t5449 + F::cast_from(0.26341796731742046394e1_f64) * t12966 * t21430;
    t72708
}
