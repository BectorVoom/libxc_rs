//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 886/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk886(t14237: f64, t16503: f64, t559: f64, t8420: f64, t14243: f64, t8425: f64, t14249: f64, t8430: f64, t34688: f64, t38560: f64, t38563: f64, t38570: f64, t38608: f64, t38610: f64, t38623: f64, t38640: f64, t38643: f64, t38645: f64, t38648: f64, t44854: f64, t44857: f64, t44860: f64, t44866: f64) -> f64 {
    let t44874 = t16503 * t14237 * t559 * t8420;
    let t44878 = t16503 * t14243 * t559 * t8425;
    let t44882 = t16503 * t14249 * t559 * t8430;
    let t44884 = 0.19211284388664477842e-2_f64 * t44854 - 0.15243824895787514157e-3_f64 * t44857 - 0.36021158228745895953e-3_f64 * t44860 - t38560 - t38563 - 0.72732431077987577943e-1_f64 * t38570 - 0.31923449919973379548e-4_f64 * t44866 - 0.30487649791575028314e-3_f64 * t38608 + 0.30487649791575028314e-3_f64 * t38610 + t38623 + t38640 - 0.59590439850616975158e-4_f64 * t38643 + 0.59590439850616975158e-4_f64 * t38645 + t38648 - t34688 - 0.25538759935978703638e-4_f64 * t44874 + 0.76616279807936110914e-4_f64 * t44878 - 0.10215503974391481455e-3_f64 * t44882;
    t44884
}
