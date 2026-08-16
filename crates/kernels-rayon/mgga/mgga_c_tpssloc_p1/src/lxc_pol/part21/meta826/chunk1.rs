//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2913/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2913(t10760: f64, t10828: f64, t14266: f64, t14329: f64, t1569: f64, t17350: f64, t17428: f64, t17499: f64, t2856: f64, t2881: f64, t2889: f64, t2906: f64, t2907: f64, t2924: f64, t2930: f64, t2932: f64, t41816: f64, t41826: f64, t41981: f64, t4411: f64, t4434: f64, t48771: f64, t48779: f64, t48890: f64, t5743: f64, t5759: f64, t5794: f64, t59975: f64, t60407: f64, t60424: f64, t60429: f64, t60434: f64, t60568: f64, t60570: f64, t60585: f64, t60601: f64, t60618: f64, t60634: f64, t60649: f64, t60665: f64, t60682: f64, t60698: f64, t924: f64, t932: f64, t950: f64) -> f64 {
    let t60711 = 0.41016075432865626631e4_f64 * t48779 * t48890 * t950 + 1.0_f64 * t17428 * t2881 + 0.32163958997385070134e2_f64 * t60407 * t2889 + 2.0_f64 * t48771 * t1569 + 4.0_f64 * t14266 * t4434 + 2.0_f64 * t4411 * t14329 - 2.0_f64 * t41981 * t5743 + 1.0_f64 * t10760 * t5759 + 2.0_f64 * t2856 * t17350 + 0.17315859105681463759e2_f64 * t41816 * t5794 - 0.11696447245269292414e1_f64 * t60424 * t2907 - t60429 + 0.34631718211362927518e2_f64 * t2930 * t59975 * t2932 - t60434 - t60568 - t60570 + 1.0_f64 * t924 * (t60585 + t60601 + t60618 + t60634 + t60649 + t60665 + t60682 + t60698) * t932 - 0.10389515463408878255e3_f64 * t10828 * t5794 * t2924 - 0.12304822629859687989e5_f64 * t41826 * t17499 * t2906;
    t60711
}
