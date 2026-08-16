//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1337/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1337<F: Float>(t1961: F, t3766: F, t5477: F, t1319: F, t3761: F, t6912: F, t1897: F, t1419: F, t16387: F, t11634: F, t21624: F, t1477: F, t21267: F) -> (F, F, F, F, F, F) {
    let t22107 = t3766 * t5477 * t1961;
    let t22111 = t3761 * t6912 * t1319;
    let t22114 = t1897 * t1961;
    let t22116 = t16387 * t22114 * t1419;
    let t22120 = t11634 * t21624 * t1319;
    let t22127 = t1477 * t21267;
    (t22107, t22111, t22114, t22116, t22120, t22127)
}
