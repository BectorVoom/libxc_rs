//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1453/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1453(t17845: f64, t460: f64, t12050: f64, t13045: f64, t3601: f64, t17710: f64, t13141: f64, t487: f64, t3603: f64, t1234: f64, t12717: f64, t12751: f64, t12756: f64, t1285: f64, t12966: f64, t12975: f64, t17188: f64, t17192: f64, t17808: f64, t17811: f64, t17815: f64, t17818: f64, t17822: f64, t17826: f64, t17829: f64, t17834: f64, t17837: f64, t17840: f64, t1818: f64, t3666: f64, t3670: f64, t3755: f64, t3756: f64, t3767: f64, t5443: f64, t5452: f64, t5463: f64) -> f64 {
    let t17846 = t460 * t17845;
    let t17847 = t12050 * t13045;
    let t17848 = t17847 * t3601;
    let t17849 = t17710 * t17848;
    let t17852 = t13141 * t487;
    let t17853 = t460 * t17852;
    let t17854 = t12050 * t3603;
    let t17855 = t17854 * t3601;
    let t17856 = t17710 * t17855;
    let t17859 = 0.26341796731742046394e1_f64 * t12717 * t17188 - 0.13170898365871023197e1_f64 * t17192 * t3756 - 0.13170898365871023197e1_f64 * t3666 * t5452 + 0.65854491829355115987e0_f64 * t460 * t17808 + 0.26341796731742046394e1_f64 * t3670 * t17811 + 0.13170898365871023197e1_f64 * t3767 * t17815 - 0.26341796731742046394e1_f64 * t12751 * t17818 - 0.13170898365871023197e1_f64 * t1234 * t17822 + 0.13170898365871023197e1_f64 * t1285 * t17826 - 0.13170898365871023197e1_f64 * t3755 * t17829 + 0.26341796731742046394e1_f64 * t12966 * t5443 - 0.13170898365871023197e1_f64 * t12751 * t17834 + 0.65854491829355115987e0_f64 * t12756 * t17837 + 0.13170898365871023197e1_f64 * t5463 * t17840 - 0.65854491829355115987e0_f64 * t12975 * t1818 + 0.39512695097613069591e1_f64 * t17846 * t17849 - 0.39512695097613069591e1_f64 * t17853 * t17856;
    t17859
}
