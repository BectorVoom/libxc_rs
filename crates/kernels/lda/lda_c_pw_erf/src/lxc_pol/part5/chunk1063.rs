//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1063/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1063<F: Float>(t15346: F, t15349: F, t11305: F, t11323: F, t19972: F, t19973: F, t19976: F, t19977: F, t19978: F, t19979: F, t19980: F, t19981: F, t19982: F, t19983: F, t8168: F, t8177: F, t8184: F, t8188: F) -> (F, F, F) {
    let t19984 = F::cast_from(1.7544670192365612_f64) * t15346;
    let t19985 = F::new(3.0) * t15349;
    let t19986 = -F::cast_from(1.4220018064581168_f64) * t11305 + t19972 - t19973 - t19976 - t8168 - t8177 - t19977 - t19978 - t19979 + t19980 + t19981 + t11323 + t8184 - t19982 - t19983 - t19984 + t19985 - t8188;
    (t19984, t19985, t19986)
}
