//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 241/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk241<F: Float>(t153: F, t274: F, t474: F, t316: F, t337: F, t341: F, t346: F, t359: F, t392: F, t394: F, t399: F, t404: F) -> (F, F) {
    let t677 = F::new(0.5694518669548363) * t153 * t474 * t274;
    let t678 = t316 + t337 + t341 - t346 + t359 + t392 + t394 - t399 - t404;
    (t677, t678)
}
