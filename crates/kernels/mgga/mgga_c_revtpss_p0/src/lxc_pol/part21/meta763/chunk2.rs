//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2711/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2711<F: Float>(t190: F, t49889: F, t706: F, t4398: F, t9387: F, t11061: F, t15071: F, t1583: F, t1940: F, t2411: F, t39442: F, t41154: F, t49872: F, t49873: F, t49877: F, t49879: F, t49882: F, t49885: F, t890: F) -> (F, F, F) {
    let t49892 = F::new(4.0) * t706 * t190 * t49889;
    let t49897 = t4398 * t9387;
    let t49898 = F::cast_from(0.5848223622634646207e0_f64) * t49897;
    let t49903 = -F::new(6.0) * t11061 * t1583 * t1940 * t41154 - F::new(3.0) * t15071 * t1940 * t2411 * t890 + t39442 + t49872 + t49873 + t49877 + t49879 + t49882 + t49885 + t49892 - t49898;
    (t49892, t49898, t49903)
}
