//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2204/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2204<F: Float>(t3151: F, t7817: F, t25698: F, t93920: F, t1096: F, t988: F, t1043: F, t1089: F, t16237: F, t16405: F, t1982: F, t1985: F, t1986: F, t25591: F, t25611: F, t25626: F, t25629: F, t27415: F, t27422: F, t27444: F, t27543: F, t27595: F, t27651: F, t3042: F, t3133: F, t3304: F, t3318: F, t4763: F, t4975: F, t7144: F, t7145: F, t7810: F, t7837: F, t93436: F, t93890: F, t93897: F, t93921: F, t94080: F, t99786: F, t999: F) -> F {
    let t99807 = t7817 * t3151;
    let t99824 = t25698 * t93920;
    let t99842 = t1096 * t988;
    let t99847 = -F::cast_from(0.8673628188205199462e0_f64) * t93897 * t99786 * t3318 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t7145 * t7810 * t3042 + F::cast_from(0.8673628188205199462e0_f64) * t25611 * t27651 * t16405 - F::cast_from(0.8673628188205199462e0_f64) * t25629 * t7817 * t3133 * t1089 - F::cast_from(0.17347256376410398924e1_f64) * t94080 * t99807 * t3304 + F::cast_from(0.8673628188205199462e0_f64) * t93890 * t99807 * t3318 + F::cast_from(0.34694512752820797848e1_f64) * t25591 * t7145 * t27444 * t999 + F::cast_from(0.34694512752820797848e1_f64) * t27415 * t27595 - F::cast_from(0.69389025505641595696e1_f64) * t93921 * t1985 * t4763 * t988 + F::cast_from(0.10408353825846239354e2_f64) * t99824 * t1985 * t4763 * t999 - F::cast_from(0.4336814094102599731e0_f64) * t1982 * t16237 * t1986 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t27422 * t1043 * t1089 - F::cast_from(0.17347256376410398924e1_f64) * t7144 * t7145 * t27543 * t988 - F::cast_from(0.8673628188205199462e0_f64) * t25626 * t7837 + F::cast_from(0.34694512752820797848e1_f64) * t93436 * t27651 * t4975 * t99842;
    t99847
}
