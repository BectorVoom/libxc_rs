//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 984/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk984<F: Float>(t12605: F, t18120: F, t1889: F, t4463: F, t4440: F, t1607: F, t5713: F, t1610: F, t5477: F, t16082: F, t6159: F, t1369: F, t531: F, t617: F) -> (F, F, F, F, F, F) {
    let t18121 = t12605 * t18120;
    let t18124 = t1889 * t4463;
    let t18125 = t4440 * t18124;
    let t18128 = t5713 * t1607;
    let t18129 = t5477 * t1610;
    let t18130 = t18128 * t18129;
    let t18133 = t6159 * t16082;
    let t18137 = t1369 * t617 * t531;
    (t18121, t18125, t18128, t18130, t18133, t18137)
}
