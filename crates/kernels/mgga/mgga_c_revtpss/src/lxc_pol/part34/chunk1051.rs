//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1051/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1051<F: Float>(t29833: F, t993: F, t378: F, t1652: F, t1696: F, t1983: F, t1986: F, t25605: F, t25611: F, t25629: F, t27550: F, t27568: F, t27621: F, t27699: F, t29728: F, t29732: F, t29740: F, t29744: F, t29748: F, t29752: F, t29760: F, t29809: F, t29812: F, t29818: F, t29822: F, t29826: F, t29830: F, t342: F, t6259: F, t7102: F, t7144: F, t7151: F, t7159: F, t7167: F, t7833: F) -> (F, F, F) {
    let t29834 = t29833 * t993;
    let t29835 = t29834 * t378;
    let t29838 = 0.17347256376410398924e1 * t7151 * t29728 + 0.8673628188205199462e0 * t7159 * t29732 - 0.65854491829355115987e0 * t7102 * t6259 - 0.13170898365871023197e1 * t27568 * t1652 - 0.17347256376410398924e1 * t25629 * t29740 + 0.17347256376410398924e1 * t25611 * t29744 - 0.17347256376410398924e1 * t7144 * t29748 - 0.26020884564615598386e1 * t7159 * t29752 - 0.13170898365871023197e1 * t27550 * t1652 - 0.8673628188205199462e0 * t27621 * t7833 + 0.17347256376410398924e1 * t25605 * t29760 + 0.65854491829355115987e0 * t342 * t29809 - 0.4336814094102599731e0 * t29812 * t1986 - 0.13170898365871023197e1 * t27699 * t1696 - 0.34694512752820797848e1 * t7151 * t29818 - 0.8673628188205199462e0 * t7167 * t29822 - 0.4336814094102599731e0 * t7167 * t29826 - 0.4336814094102599731e0 * t1983 * t29830 - 0.8673628188205199462e0 * t29835 * t1986;
    (t29834, t29835, t29838)
}
