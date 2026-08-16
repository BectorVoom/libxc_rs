//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1222/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1222<F: Float>(t10944: F, t2099: F, t5933: F, t1107: F, t21267: F, t26211: F, t721: F, t2866: F, t9242: F, t20683: F, t9229: F, t1899: F, t2782: F, t3525: F) -> (F, F, F, F, F) {
    let t30164 = t5933 * t2099 * t10944;
    let t30193 = F::cast_from(0.30762056574649219973e4_f64) * t21267 * t26211 * t1107 * t721;
    let t30195 = F::cast_from(0.35089341735807877242e1_f64) * t9242 * t2866;
    let t30197 = F::cast_from(0.2894756309764656312e3_f64) * t20683 * t9229;
    let t30200 = F::cast_from(18.0_f64) * t1899 * t3525 * t2782;
    (t30164, t30193, t30195, t30197, t30200)
}
