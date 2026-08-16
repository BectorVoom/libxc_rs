//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2234/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2234(t11223: f64, t7143: f64, t3057: f64, t7810: f64, t11120: f64, t994: f64, t1096: f64, t11213: f64, t1696: f64, t1985: f64, t25464: f64, t25480: f64, t25617: f64, t25640: f64, t25699: f64, t27419: f64, t27556: f64, t27568: f64, t27609: f64, t27627: f64, t27631: f64, t27692: f64, t27699: f64, t27703: f64, t3060: f64, t3067: f64, t3075: f64, t3270: f64, t3326: f64, t4946: f64, t7145: f64, t7151: f64, t7159: f64, t7160: f64, t7818: f64, t7821: f64, t93867: f64, t93928: f64, t94095: f64, t988: f64) -> (f64, f64) {
    let t100658 = t11223 * t7143;
    let t100681 = t3057 * t7810;
    let t100690 = t7143 * t11120;
    let t100691 = t994 * t100690;
    let t100696 = -0.8673628188205199462e0_f64 * t11213 * t7143 * t7818 - 0.26020884564615598386e1_f64 * t25699 * t7145 * t7821 * t3075 + 0.34694512752820797848e1_f64 * t100658 * t27703 - 0.8673628188205199462e0_f64 * t25640 * t27627 - 0.8673628188205199462e0_f64 * t25640 * t27631 - 0.34694512752820797848e1_f64 * t7151 * t7160 * t27556 * t1096 - 0.65854491829355115987e0_f64 * t93867 * t1696 - 0.52041769129231196772e1_f64 * t93928 * t27692 + 0.8673628188205199462e0_f64 * t27609 * t25480 + 0.17347256376410398924e1_f64 * t27419 * t25617 - 0.26020884564615598386e1_f64 * t7159 * t25464 * t7810 * t3270 + 0.13170898365871023197e1_f64 * t100681 * t3060 - 0.65854491829355115987e0_f64 * t27699 * t3326 + 0.13170898365871023197e1_f64 * t27568 * t3067 + 0.34694512752820797848e1_f64 * t94095 * t27703 - 0.10408353825846239354e2_f64 * t100691 * t1985 * t4946 * t988;
    (t100690, t100696)
}
