//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2203/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2203<F: Float>(t1668: F, t7135: F, t3153: F, t1976: F, t4866: F, t1035: F, t1983: F, t99682: F, t73: F, t3151: F, t7821: F, t1043: F, t1089: F, t1096: F, t16568: F, t16573: F, t25461: F, t25476: F, t25601: F, t25605: F, t25611: F, t27411: F, t27422: F, t27423: F, t27426: F, t27640: F, t27642: F, t27661: F, t27664: F, t27684: F, t3133: F, t3304: F, t4910: F, t4982: F, t4997: F, t4998: F, t7144: F, t7151: F, t7160: F, t93437: F, t93890: F, t93897: F, t93983: F, t94085: F, t99685: F, t999: F) -> (F, F, F, F, F, F) {
    let t99729 = t7135 * t1668;
    let t99730 = t99729 * t3153;
    let t99734 = t1976 * t4866;
    let t99735 = t99734 * t3153;
    let t99743 = t1983 * t99682 * t1035;
    let t99762 = t99734 * t73;
    let t99786 = t7821 * t3151;
    let t99790 = F::cast_from(0.8673628188205199462e0_f64) * t27640 * t99730 * t4998 + F::cast_from(0.8673628188205199462e0_f64) * t27640 * t99735 * t4998 + F::cast_from(0.4336814094102599731e0_f64) * t27640 * t27642 * t16573 - F::cast_from(0.4336814094102599731e0_f64) * t99743 * t99685 * t16568 + F::cast_from(0.34694512752820797848e1_f64) * t93983 * t27642 * t4982 * t1096 * t1043 - F::cast_from(0.17347256376410398924e1_f64) * t25476 * t27423 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t27426 * t1043 * t1089 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t7160 * t27411 * t999 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t99762 * t27664 + F::cast_from(0.17347256376410398924e1_f64) * t93890 * t27642 * t4997 * t93437 - F::cast_from(0.17347256376410398924e1_f64) * t93897 * t27642 * t4997 * t4910 + F::cast_from(0.34694512752820797848e1_f64) * t27661 * t25601 + F::cast_from(0.34694512752820797848e1_f64) * t7144 * t7160 * t27422 * t1096 - F::cast_from(0.34694512752820797848e1_f64) * t25461 * t27684 + F::cast_from(0.8673628188205199462e0_f64) * t25611 * t7821 * t3133 * t1089 + F::cast_from(0.17347256376410398924e1_f64) * t94085 * t99786 * t3304;
    (t99729, t99730, t99735, t99762, t99786, t99790)
}
