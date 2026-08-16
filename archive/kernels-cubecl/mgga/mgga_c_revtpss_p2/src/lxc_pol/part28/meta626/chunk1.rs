//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2236/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2236<F: Float>(t1096: F, t1646: F, t1000: F, t100490: F, t100698: F, t100702: F, t100705: F, t100708: F, t100723: F, t100737: F, t1097: F, t16275: F, t1985: F, t25473: F, t25591: F, t25701: F, t27412: F, t27433: F, t27653: F, t27668: F, t27669: F, t27670: F, t27679: F, t27702: F, t3042: F, t3318: F, t4983: F, t64841: F, t7144: F, t7145: F, t7156: F, t7160: F, t7162: F, t7828: F, t93498: F, t93502: F, t93921: F, t94026: F, t94042: F, t94063: F, t99730: F, t999: F) -> F {
    let t100743 = t1646 * t1096;
    let t100748 = F::cast_from(0.10408353825846239354e2_f64) * t100698 * t1985 * t64841 - F::cast_from(0.13170898365871023197e1_f64) * t100702 * t1097 - F::cast_from(0.26020884564615598386e1_f64) * t100705 * t25701 - F::cast_from(0.13170898365871023197e1_f64) * t100708 * t1000 + F::cast_from(0.34694512752820797848e1_f64) * t25591 * t7145 * t27679 * t999 + F::cast_from(0.17347256376410398924e1_f64) * t7144 * t7160 * t7828 * t3042 + F::cast_from(0.17347256376410398924e1_f64) * t25473 * t27412 - F::cast_from(0.39512695097613069591e1_f64) * t94026 * t16275 + F::cast_from(0.17347256376410398924e1_f64) * t100723 * t7162 - F::cast_from(0.17347256376410398924e1_f64) * t7156 * t27668 * t27670 - F::cast_from(0.17347256376410398924e1_f64) * t27669 * t99730 * t4983 - F::cast_from(0.8673628188205199462e0_f64) * t94063 * t100490 * t3318 - F::cast_from(0.17347256376410398924e1_f64) * t94042 * t27653 - F::cast_from(0.17347256376410398924e1_f64) * t100737 * t27433 + F::cast_from(0.34694512752820797848e1_f64) * t93502 * t27702 * t93498 - F::cast_from(0.69389025505641595696e1_f64) * t93921 * t1985 * t100743 * t999;
    t100748
}
