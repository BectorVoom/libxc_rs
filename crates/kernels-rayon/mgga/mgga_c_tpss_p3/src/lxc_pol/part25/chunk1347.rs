//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1347/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1347(t13880: f64, t13940: f64, t1656: f64, t1838: f64, t1842: f64, t18483: f64, t18490: f64, t18496: f64, t18950: f64, t18967: f64, t19540: f64, t19554: f64, t20154: f64, t20190: f64, t21074: f64, t21823: f64, t21831: f64, t21852: f64, t4516: f64, t51545: f64, t5433: f64, t5737: f64, t5739: f64, t5740: f64, t5921: f64, t5925: f64, t60649: f64, t60653: f64, t62508: f64, t6424: f64, t6425: f64, t65667: f64, t66970: f64, t69452: f64, t69458: f64, t69681: f64, t69699: f64, t69727: f64, t69730: f64, t69734: f64, t69741: f64) -> f64 {
    let t71715 = 2.0_f64 * t18483 * t21831 - 12.0_f64 * t5739 * t18490 * t6424 * t4516 + 2.0_f64 * t18950 * t5433 - 2.0_f64 * t18496 * t18967 * t69699 + 6.0_f64 * t60653 * t18967 * t69681 - 4.0_f64 * t60649 * t21823 - 4.0_f64 * t18496 * t62508 * t21074 - t5737 * t21852 - t69452 * t1842 + 4.0_f64 * t5739 * t5740 * t20154 * t1656 + 4.0_f64 * t65667 * t6425 + 2.0_f64 * t5739 * t5740 * t1838 * t13940 + 2.0_f64 * t69458 * t5925 - 2.0_f64 * t19540 * t20190 * t51545 + 2.0_f64 * t19540 * t66970 * t19554 - 2.0_f64 * t18496 * t18967 * t69741 - 6.0_f64 * t5921 * t13880 + t19540 * t18967 * t69727 - 2.0_f64 * t18496 * t18967 * t69730 + 2.0_f64 * t19540 * t18967 * t69734;
    t71715
}
