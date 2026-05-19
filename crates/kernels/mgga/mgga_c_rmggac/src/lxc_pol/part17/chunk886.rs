//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 886/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk886<F: Float>(t14237: F, t16503: F, t559: F, t8420: F, t14243: F, t8425: F, t14249: F, t8430: F, t34688: F, t38560: F, t38563: F, t38570: F, t38608: F, t38610: F, t38623: F, t38640: F, t38643: F, t38645: F, t38648: F, t44854: F, t44857: F, t44860: F, t44866: F) -> F {
    let t44874 = t16503 * t14237 * t559 * t8420;
    let t44878 = t16503 * t14243 * t559 * t8425;
    let t44882 = t16503 * t14249 * t559 * t8430;
    let t44884 = F::cast_from(0.19211284388664477842e-2_f64) * t44854 - F::cast_from(0.15243824895787514157e-3_f64) * t44857 - F::cast_from(0.36021158228745895953e-3_f64) * t44860 - t38560 - t38563 - F::cast_from(0.72732431077987577943e-1_f64) * t38570 - F::cast_from(0.31923449919973379548e-4_f64) * t44866 - F::cast_from(0.30487649791575028314e-3_f64) * t38608 + F::cast_from(0.30487649791575028314e-3_f64) * t38610 + t38623 + t38640 - F::cast_from(0.59590439850616975158e-4_f64) * t38643 + F::cast_from(0.59590439850616975158e-4_f64) * t38645 + t38648 - t34688 - F::cast_from(0.25538759935978703638e-4_f64) * t44874 + F::cast_from(0.76616279807936110914e-4_f64) * t44878 - F::cast_from(0.10215503974391481455e-3_f64) * t44882;
    t44884
}
