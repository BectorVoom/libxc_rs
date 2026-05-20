//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1945/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1945<F: Float>(t1984: F, t29807: F, t359: F, t1646: F, t993: F, t378: F, t1652: F, t1696: F, t1983: F, t1986: F, t25605: F, t25611: F, t25629: F, t27550: F, t27568: F, t27621: F, t27699: F, t29728: F, t29732: F, t29740: F, t29744: F, t29748: F, t29752: F, t29760: F, t29809: F, t29812: F, t29818: F, t29822: F, t29826: F, t342: F, t6259: F, t7102: F, t7144: F, t7151: F, t7159: F, t7167: F, t7833: F) -> (F, F, F, F, F) {
    let t29830 = t1984 * t359 * t29807;
    let t29833 = t1646 * t1646;
    let t29834 = t29833 * t993;
    let t29835 = t29834 * t378;
    let t29838 = F::cast_from(0.17347256376410398924e1_f64) * t7151 * t29728 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t29732 - F::cast_from(0.65854491829355115987e0_f64) * t7102 * t6259 - F::cast_from(0.13170898365871023197e1_f64) * t27568 * t1652 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t29740 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t29744 - F::cast_from(0.17347256376410398924e1_f64) * t7144 * t29748 - F::cast_from(0.26020884564615598386e1_f64) * t7159 * t29752 - F::cast_from(0.13170898365871023197e1_f64) * t27550 * t1652 - F::cast_from(0.8673628188205199462e0_f64) * t27621 * t7833 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t29760 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t29809 - F::cast_from(0.4336814094102599731e0_f64) * t29812 * t1986 - F::cast_from(0.13170898365871023197e1_f64) * t27699 * t1696 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t29818 - F::cast_from(0.8673628188205199462e0_f64) * t7167 * t29822 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t29826 - F::cast_from(0.4336814094102599731e0_f64) * t1983 * t29830 - F::cast_from(0.8673628188205199462e0_f64) * t29835 * t1986;
    (t29830, t29833, t29834, t29835, t29838)
}
