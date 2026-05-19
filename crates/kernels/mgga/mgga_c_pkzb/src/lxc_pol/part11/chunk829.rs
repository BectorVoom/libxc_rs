//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 829/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk829<F: Float>(t7033: F, t7038: F, t7040: F, t5179: F, t5187: F, t4996: F, t5005: F, t5011: F, t5019: F, t5022: F, t5154: F, t5186: F, t7030: F, t7037: F, t7042: F, t8795: F) -> (F, F, F, F, F, F) {
    let t8842 = F::cast_from(0.34631718211362927517e2_f64) * t7033;
    let t8843 = F::cast_from(0.11696447245269292414e1_f64) * t7038;
    let t8844 = F::cast_from(0.23392894490538584828e1_f64) * t7040;
    let t8845 = F::new(12.0) * t5179;
    let t8846 = F::new(32.0) * t5187;
    let t8847 = t7030 - t5154 - t8795 + t4996 + t5005 - t5011 - t8842 - t7037 - t8843 + t8844 + t5019 - t5022 - t7042 + t8845 + t5186 + t8846;
    (t8842, t8843, t8844, t8845, t8846, t8847)
}
