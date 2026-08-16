//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3786/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3786(t3140: f64, t3566: f64, t13147: f64, t1811: f64, t460: f64, t1243: f64, t12699: f64, t12751: f64, t1287: f64, t12966: f64, t16697: f64, t16751: f64, t16763: f64, t17289: f64, t1774: f64, t17818: f64, t17826: f64, t17849: f64, t17861: f64, t17921: f64, t20795: f64, t21430: f64, t3584: f64, t3596: f64, t3755: f64, t3781: f64, t3782: f64, t3783: f64, t45710: f64, t487: f64, t5326: f64, t5412: f64, t5436: f64, t5449: f64, t5464: f64, t5474: f64, t5481: f64, t6727: f64, t6731: f64, t70693: f64, t72359: f64) -> f64 {
    let t72661 = t3566 * t3140;
    let t72686 = t460 * t13147 * t1811;
    let t72708 = 0.10536718692696818558e2_f64 * t72661 * t3596 * t487 * t1774 * t17818 - 0.52683593463484092788e1_f64 * t72661 * t1243 * t487 * t1774 * t16697 - 0.65854491829355115987e0_f64 * t3782 * t72359 * t3783 - 0.13170898365871023197e1_f64 * t12751 * t20795 * t5464 * t3584 - 0.13170898365871023197e1_f64 * t5326 * t16751 - 0.26341796731742046394e1_f64 * t460 * t3781 * t5412 * t5481 + 0.79025390195226139182e1_f64 * t72686 * t17849 + 0.13170898365871023197e1_f64 * t5436 * t17921 - 0.26341796731742046394e1_f64 * t3755 * t70693 * t1287 + 0.13170898365871023197e1_f64 * t45710 * t6727 + 0.26341796731742046394e1_f64 * t5436 * t17826 + 0.13170898365871023197e1_f64 * t12699 * t6731 + 0.13170898365871023197e1_f64 * t5436 * t16763 + 0.26341796731742046394e1_f64 * t17861 * t5474 - 0.26341796731742046394e1_f64 * t17289 * t5449 + 0.26341796731742046394e1_f64 * t12966 * t21430;
    t72708
}
