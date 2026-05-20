//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2111/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2111<F: Float>(t25878: F, t98028: F, t94771: F, t97814: F, t1903: F, t25931: F, t1882: F, t2027: F, t2028: F, t25889: F, t25933: F, t26034: F, t26084: F, t27837: F, t27868: F, t49376: F, t543: F, t545: F, t5775: F, t7295: F, t7296: F, t7301: F, t94823: F, t94880: F, t94882: F, t94884: F, t94887: F, t94891: F, t94895: F, t98290: F) -> F {
    let t98333 = t25878 * t98028;
    let t98338 = t94771 * t97814;
    let t98340 = t25931 * t1903;
    let t98353 = -F::cast_from(0.4336814094102599731e0_f64) * t2027 * t2028 * t545 * t98290 + F::cast_from(0.8673628188205199462e0_f64) * t27868 * t25931 * t49376 - F::cast_from(0.12851425765524037203e-1_f64) * t94880 - F::cast_from(0.12851425765524037203e-1_f64) * t94882 + F::cast_from(0.2601984143835408805e-1_f64) * t94884 - F::cast_from(0.13170898365871023197e1_f64) * t26084 * t5775 + F::cast_from(0.51405703062096148812e-1_f64) * t94887 - F::cast_from(0.28912093960683998208e-1_f64) * t94891 - F::cast_from(0.34270468708064099208e-1_f64) * t98333 + F::cast_from(0.72280234901709995518e-2_f64) * t94895 + F::cast_from(0.17347256376410398924e1_f64) * t27837 * t25889 - F::cast_from(0.3427046870806409921e-2_f64) * t98338 + F::cast_from(0.52041769129231196772e1_f64) * t94823 * t98340 * t25933 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t26034 * t1903 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t26034 * t1882 * t543;
    t98353
}
