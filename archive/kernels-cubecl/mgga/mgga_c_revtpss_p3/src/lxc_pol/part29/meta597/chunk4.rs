//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2019/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2019<F: Float>(t2439: F, t780: F, t785: F, t7997: F, t103352: F, t103364: F, t15038: F, t1558: F, t1580: F, t213: F, t225: F, t25391: F, t25394: F, t25407: F, t257: F, t26441: F, t26550: F, t27199: F, t7403: F, t8016: F, t95832: F, t95834: F, t95836: F, t95847: F, t95855: F, t95857: F, t95894: F) -> F {
    let t103370 = t2439 * t785 * t7997 * t780;
    let t103380 = F::cast_from(0.13170898365871023197e1_f64) * t7403 * t15038 - F::cast_from(0.28912093960683998208e-1_f64) * t95832 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t103352 * t225 * t257 + F::cast_from(0.12851425765524037203e-1_f64) * t95834 - F::cast_from(0.34270468708064099208e-2_f64) * t95836 + F::cast_from(0.10975748638225852664e-1_f64) * t95847 - F::cast_from(0.25702851531048074406e-1_f64) * t95855 + F::cast_from(0.14456046980341999104e-1_f64) * t95857 + F::cast_from(0.17135234354032049604e-2_f64) * t103364 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t26441 - F::cast_from(0.65049603595885220126e-3_f64) * t103370 - F::cast_from(0.4336814094102599731e0_f64) * t25407 * t8016 - F::cast_from(0.65854491829355115987e0_f64) * t95894 * t1580 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t26550 * t1558 * t25394;
    t103380
}
