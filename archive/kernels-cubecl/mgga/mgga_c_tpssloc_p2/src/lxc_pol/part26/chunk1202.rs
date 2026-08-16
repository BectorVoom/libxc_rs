//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1202/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1202<F: Float>(t2314: F, t23831: F, t22607: F, t7000: F, t6880: F, t22592: F, t6876: F, t22949: F, t12020: F, t225: F, t12022: F, t1985: F, t6889: F) -> (F, F, F, F, F, F) {
    let t80627 = F::cast_from(6.0_f64) * t2314 * t23831;
    let t80629 = F::cast_from(3.0_f64) * t22607 * t7000;
    let t80633 = F::cast_from(9.0_f64) * t22607 * t6880;
    let t80635 = F::cast_from(18.0_f64) * t6876 * t22592;
    let t80637 = F::cast_from(3.0_f64) * t6876 * t22949;
    let t80640 = t225 * t12020;
    let t80643 = t1985 * t6889 * t80640 * t12022;
    (t80627, t80629, t80633, t80635, t80637, t80643)
}
