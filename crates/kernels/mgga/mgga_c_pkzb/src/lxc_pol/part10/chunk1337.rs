//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1337/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1337<F: Float>(t1133: F, t2916: F, t2019: F, t3669: F, t2036: F, t26659: F, t1138: F, t18319: F, t2123: F, t2127: F, t2128: F, t22063: F, t25117: F, t25122: F, t2971: F, t2981: F, t2984: F, t5718: F, t5931: F, t5952: F, t6023: F, t7831: F, t7832: F, t7833: F, t7845: F, t7858: F, t9319: F, t9661: F, t9674: F, t9675: F, t9704: F) -> (F,) {
    let t26677 = t1133 * t2916;
    let t26687 = t2019 * t3669;
    let t26695 = t2036 * t26659;
    let t26714 = 0.52683593463484092788e1 * t2019 * t26677 * t2971 + 0.15805078039045227836e2 * t7831 * t7832 * t9319 * t2916 + 0.13170898365871023197e1 * t2984 * t7858 + 0.13170898365871023197e1 * t26687 * t2128 + 0.26341796731742046394e1 * t2123 * t25122 * t2127 + 0.13170898365871023197e1 * t22063 * t1138 - 0.13170898365871023197e1 * t26695 * t2981 + 0.26341796731742046394e1 * t9675 * t6023 + 0.79025390195226139182e1 * t5952 * t9674 * t7833 - 0.79025390195226139182e1 * t5718 * t9674 * t7845 - 0.26341796731742046394e1 * t2036 * t26677 * t2981 + 0.39512695097613069591e1 * t9661 * t18319 + 0.13170898365871023197e1 * t5931 * t25117 * t9704;
    (t26714,)
}
