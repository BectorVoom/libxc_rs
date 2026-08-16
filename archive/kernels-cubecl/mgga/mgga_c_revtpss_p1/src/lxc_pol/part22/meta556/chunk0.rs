//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2382/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2382<F: Float>(t17845: F, t460: F, t12050: F, t13045: F, t3601: F, t17710: F, t13141: F, t487: F, t3603: F, t1234: F, t12717: F, t12751: F, t12756: F, t1285: F, t12966: F, t12975: F, t17188: F, t17192: F, t17808: F, t17811: F, t17815: F, t17818: F, t17822: F, t17826: F, t17829: F, t17834: F, t17837: F, t17840: F, t1818: F, t3666: F, t3670: F, t3755: F, t3756: F, t3767: F, t5443: F, t5452: F, t5463: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17846 = t460 * t17845;
    let t17847 = t12050 * t13045;
    let t17848 = t17847 * t3601;
    let t17849 = t17710 * t17848;
    let t17852 = t13141 * t487;
    let t17853 = t460 * t17852;
    let t17854 = t12050 * t3603;
    let t17855 = t17854 * t3601;
    let t17856 = t17710 * t17855;
    let t17859 = F::cast_from(0.26341796731742046394e1_f64) * t12717 * t17188 - F::cast_from(0.13170898365871023197e1_f64) * t17192 * t3756 - F::cast_from(0.13170898365871023197e1_f64) * t3666 * t5452 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t17808 + F::cast_from(0.26341796731742046394e1_f64) * t3670 * t17811 + F::cast_from(0.13170898365871023197e1_f64) * t3767 * t17815 - F::cast_from(0.26341796731742046394e1_f64) * t12751 * t17818 - F::cast_from(0.13170898365871023197e1_f64) * t1234 * t17822 + F::cast_from(0.13170898365871023197e1_f64) * t1285 * t17826 - F::cast_from(0.13170898365871023197e1_f64) * t3755 * t17829 + F::cast_from(0.26341796731742046394e1_f64) * t12966 * t5443 - F::cast_from(0.13170898365871023197e1_f64) * t12751 * t17834 + F::cast_from(0.65854491829355115987e0_f64) * t12756 * t17837 + F::cast_from(0.13170898365871023197e1_f64) * t5463 * t17840 - F::cast_from(0.65854491829355115987e0_f64) * t12975 * t1818 + F::cast_from(0.39512695097613069591e1_f64) * t17846 * t17849 - F::cast_from(0.39512695097613069591e1_f64) * t17853 * t17856;
    (t17846, t17847, t17848, t17849, t17852, t17853, t17854, t17855, t17856, t17859)
}
