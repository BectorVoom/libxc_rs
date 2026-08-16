//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2204/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2204(t3151: f64, t7817: f64, t25698: f64, t93920: f64, t1096: f64, t988: f64, t1043: f64, t1089: f64, t16237: f64, t16405: f64, t1982: f64, t1985: f64, t1986: f64, t25591: f64, t25611: f64, t25626: f64, t25629: f64, t27415: f64, t27422: f64, t27444: f64, t27543: f64, t27595: f64, t27651: f64, t3042: f64, t3133: f64, t3304: f64, t3318: f64, t4763: f64, t4975: f64, t7144: f64, t7145: f64, t7810: f64, t7837: f64, t93436: f64, t93890: f64, t93897: f64, t93921: f64, t94080: f64, t99786: f64, t999: f64) -> f64 {
    let t99807 = t7817 * t3151;
    let t99824 = t25698 * t93920;
    let t99842 = t1096 * t988;
    let t99847 = -0.8673628188205199462e0_f64 * t93897 * t99786 * t3318 - 0.8673628188205199462e0_f64 * t7144 * t7145 * t7810 * t3042 + 0.8673628188205199462e0_f64 * t25611 * t27651 * t16405 - 0.8673628188205199462e0_f64 * t25629 * t7817 * t3133 * t1089 - 0.17347256376410398924e1_f64 * t94080 * t99807 * t3304 + 0.8673628188205199462e0_f64 * t93890 * t99807 * t3318 + 0.34694512752820797848e1_f64 * t25591 * t7145 * t27444 * t999 + 0.34694512752820797848e1_f64 * t27415 * t27595 - 0.69389025505641595696e1_f64 * t93921 * t1985 * t4763 * t988 + 0.10408353825846239354e2_f64 * t99824 * t1985 * t4763 * t999 - 0.4336814094102599731e0_f64 * t1982 * t16237 * t1986 - 0.17347256376410398924e1_f64 * t25629 * t27422 * t1043 * t1089 - 0.17347256376410398924e1_f64 * t7144 * t7145 * t27543 * t988 - 0.8673628188205199462e0_f64 * t25626 * t7837 + 0.34694512752820797848e1_f64 * t93436 * t27651 * t4975 * t99842;
    t99847
}
