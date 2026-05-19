//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 734/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk734<F: Float>(t2030: F, t5728: F, t1854: F, t659: F, t5519: F, t1898: F, t1897: F, t224: F, t212: F) -> (F, F, F, F, F, F, F) {
    let t5729 = t5728 * t2030;
    let t5734 = t659 * t1854;
    let t5745 = F::cast_from(0.55403703703703703703e-1_f64) * t5519;
    let t5758 = F::cast_from(0.28842592592592592592e-1_f64) * t5519;
    let t5771 = t659 * t1898;
    let t5775 = F::new(1.0) / t1897 / t224;
    let t5776 = t212 * t5775;
    (t5729, t5734, t5745, t5758, t5771, t5775, t5776)
}
