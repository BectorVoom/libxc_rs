//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2234/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2234<F: Float>(t11223: F, t7143: F, t3057: F, t7810: F, t11120: F, t994: F, t1096: F, t11213: F, t1696: F, t1985: F, t25464: F, t25480: F, t25617: F, t25640: F, t25699: F, t27419: F, t27556: F, t27568: F, t27609: F, t27627: F, t27631: F, t27692: F, t27699: F, t27703: F, t3060: F, t3067: F, t3075: F, t3270: F, t3326: F, t4946: F, t7145: F, t7151: F, t7159: F, t7160: F, t7818: F, t7821: F, t93867: F, t93928: F, t94095: F, t988: F) -> (F, F) {
    let t100658 = t11223 * t7143;
    let t100681 = t3057 * t7810;
    let t100690 = t7143 * t11120;
    let t100691 = t994 * t100690;
    let t100696 = -F::cast_from(0.8673628188205199462e0_f64) * t11213 * t7143 * t7818 - F::cast_from(0.26020884564615598386e1_f64) * t25699 * t7145 * t7821 * t3075 + F::cast_from(0.34694512752820797848e1_f64) * t100658 * t27703 - F::cast_from(0.8673628188205199462e0_f64) * t25640 * t27627 - F::cast_from(0.8673628188205199462e0_f64) * t25640 * t27631 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t7160 * t27556 * t1096 - F::cast_from(0.65854491829355115987e0_f64) * t93867 * t1696 - F::cast_from(0.52041769129231196772e1_f64) * t93928 * t27692 + F::cast_from(0.8673628188205199462e0_f64) * t27609 * t25480 + F::cast_from(0.17347256376410398924e1_f64) * t27419 * t25617 - F::cast_from(0.26020884564615598386e1_f64) * t7159 * t25464 * t7810 * t3270 + F::cast_from(0.13170898365871023197e1_f64) * t100681 * t3060 - F::cast_from(0.65854491829355115987e0_f64) * t27699 * t3326 + F::cast_from(0.13170898365871023197e1_f64) * t27568 * t3067 + F::cast_from(0.34694512752820797848e1_f64) * t94095 * t27703 - F::cast_from(0.10408353825846239354e2_f64) * t100691 * t1985 * t4946 * t988;
    (t100690, t100696)
}
