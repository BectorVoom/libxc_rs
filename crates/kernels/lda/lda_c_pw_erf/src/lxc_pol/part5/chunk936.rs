//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 936/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk936<F: Float>(t11307: F, t11309: F, t1: F, t397: F, t7376: F, t8180: F, t11313: F, t11315: F, t11317: F, t11319: F, t15341: F, t15344: F, t15346: F, t15349: F, t11305: F, t11323: F, t8168: F, t8177: F, t8184: F, t8188: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19972 = 10.526802115419367 * t11307;
    let t19973 = 155.84180309438278 * t11309;
    let t19975 = t7376 * t1 * t397;
    let t19976 = 0.0001831155503675316 * t19975;
    let t19977 = 1025.3897021007795 * t8180;
    let t19978 = 0.06506148529668915 * t11313;
    let t19979 = 0.09759222794503372 * t11315;
    let t19980 = 0.04879611397251686 * t11317;
    let t19981 = 1.4447833828541736 * t11319;
    let t19982 = 51.94726769812759 * t15341;
    let t19983 = 1.7544670192365612 * t15344;
    let t19984 = 1.7544670192365612 * t15346;
    let t19985 = 3.0 * t15349;
    let t19986 = -1.4220018064581168 * t11305 + t19972 - t19973 - t19976 - t8168 - t8177 - t19977 - t19978 - t19979 + t19980 + t19981 + t11323 + t8184 - t19982 - t19983 - t19984 + t19985 - t8188;
    (t19972, t19973, t19976, t19977, t19978, t19979, t19980, t19981, t19982, t19983, t19984, t19985, t19986)
}
