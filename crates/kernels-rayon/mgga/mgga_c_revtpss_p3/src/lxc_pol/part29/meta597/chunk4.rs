//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2019/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2019(t2439: f64, t780: f64, t785: f64, t7997: f64, t103352: f64, t103364: f64, t15038: f64, t1558: f64, t1580: f64, t213: f64, t225: f64, t25391: f64, t25394: f64, t25407: f64, t257: f64, t26441: f64, t26550: f64, t27199: f64, t7403: f64, t8016: f64, t95832: f64, t95834: f64, t95836: f64, t95847: f64, t95855: f64, t95857: f64, t95894: f64) -> f64 {
    let t103370 = t2439 * t785 * t7997 * t780;
    let t103380 = 0.13170898365871023197e1_f64 * t7403 * t15038 - 0.28912093960683998208e-1_f64 * t95832 + 0.65854491829355115987e0_f64 * t213 * t103352 * t225 * t257 + 0.12851425765524037203e-1_f64 * t95834 - 0.34270468708064099208e-2_f64 * t95836 + 0.10975748638225852664e-1_f64 * t95847 - 0.25702851531048074406e-1_f64 * t95855 + 0.14456046980341999104e-1_f64 * t95857 + 0.17135234354032049604e-2_f64 * t103364 + 0.8673628188205199462e0_f64 * t27199 * t26441 - 0.65049603595885220126e-3_f64 * t103370 - 0.4336814094102599731e0_f64 * t25407 * t8016 - 0.65854491829355115987e0_f64 * t95894 * t1580 - 0.17347256376410398924e1_f64 * t25391 * t26550 * t1558 * t25394;
    t103380
}
