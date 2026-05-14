//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1195/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1195<F: Float>(t2732: F, t8189: F, t14294: F, t2282: F, t9848: F, t4170: F, t8286: F, t488: F, t8232: F, t33643: F, t9836: F, t33676: F, t9839: F, t6332: F, t8072: F, t32260: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34843 = t2732 * t8189;
    let t34845 = 6.0 * t14294 * t34843;
    let t34846 = t9848 * t2282;
    let t34848 = 4.0 * t4170 * t34846;
    let t34849 = t2732 * t8286;
    let t34851 = 2.0 * t4170 * t34849;
    let t34852 = t8232 * t488;
    let t34854 = t33643 * t9836;
    let t34856 = t33676 * t9839;
    let t34858 = t6332 * t8072;
    let t34859 = t32260 * t34858;
    (t34843, t34845, t34846, t34848, t34849, t34851, t34852, t34854, t34856, t34858, t34859)
}
